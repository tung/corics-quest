mod actor;
mod aseprite;
mod async_utils;
mod contexts;
mod direction;
mod enemy;
mod input;
mod ldtk;
mod levels;
mod meter;
mod modes;
mod progress;
mod random;
mod resources;
mod script;
mod shaders;
mod sprite;
mod text;
mod window;

use actor::*;
use async_utils::*;
use contexts::*;
use input::*;
use levels::*;
use modes::*;
use resources::*;
use shaders::*;

use miniquad::graphics::{
    Bindings, Buffer, BufferType, GraphicsContext, Pipeline, RenderPass, Texture, TextureFormat,
    TextureParams,
};
use miniquad::{EventHandler, KeyCode, KeyMods};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 176;
const FRAME_TIME: f64 = 1.0 / 60.0;
const MAX_FRAME_TIME: f64 = FRAME_TIME * 4.0;

struct App {
    script: Pin<Box<dyn Future<Output = ()>>>,
    dummy_waker: Waker,
    last_time: f64,
    time_bank: f64,
    offscreen_pass: RenderPass,
    screen_pipeline: Pipeline,
    screen_bindings: Bindings,
    window_width: f32,
    window_height: f32,
    gctx_ptr: SharedMut<*mut GraphicsContext>,
    input: SharedMut<Input>,
    modes: SharedMut<ModeStack>,
    level: SharedMut<Level>,
    actors: SharedMut<Vec<Actor>>,
    fade: SharedMut<[f32; 4]>,
}

impl App {
    fn new(gctx: &mut GraphicsContext) -> Self {
        let offscreen_texture = Texture::new_render_texture(
            gctx,
            TextureParams {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let offscreen_pass = RenderPass::new(gctx, offscreen_texture, None);

        let screen_pipeline = screen_shader::pipeline(gctx);

        let quad_vbuf = Buffer::immutable(
            gctx,
            BufferType::VertexBuffer,
            &[[0.0f32, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
        );
        let quad_ibuf = Buffer::immutable(gctx, BufferType::IndexBuffer, &[0u16, 2, 1, 1, 2, 3]);
        let screen_bindings = Bindings {
            vertex_buffers: vec![quad_vbuf],
            index_buffer: quad_ibuf,
            images: vec![offscreen_texture],
        };

        let res = Resources::new(gctx, quad_vbuf, quad_ibuf);

        let gctx_ptr = SharedMut::new(std::ptr::null_mut());
        let input = SharedMut::new(Input::new());
        let modes = SharedMut::new(ModeStack::new());
        let (level, actors) = {
            let (level, mut actors) = res.levels.level_by_identifier(gctx, &res, "Start");
            let mut player = Actor::new(gctx, &res, ActorType::Player, 6, 3, "coric.png");
            player.start_animation("face_s");
            actors.insert(0, player);
            (SharedMut::new(level), SharedMut::new(actors))
        };
        let fade = SharedMut::new([0.0; 4]);
        let sctx = ScriptContext::new(&gctx_ptr, res, &input, &modes, &level, &actors, &fade);

        Self {
            script: Box::pin(script::script_main(sctx)),
            dummy_waker: async_utils::new_dummy_waker(),
            last_time: 0.0,
            time_bank: 0.0,
            offscreen_pass,
            screen_pipeline,
            screen_bindings,
            window_width: SCREEN_WIDTH as f32,
            window_height: SCREEN_HEIGHT as f32,
            gctx_ptr,
            input,
            modes,
            level,
            actors,
            fade,
        }
    }

    fn window_scale(&self) -> [f32; 2] {
        let window_aspect = self.window_width / self.window_height;
        let desired_aspect = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
        if window_aspect < desired_aspect {
            [1.0, window_aspect / desired_aspect]
        } else {
            [desired_aspect / window_aspect, 1.0]
        }
    }
}

impl EventHandler for App {
    fn draw(&mut self, gctx: &mut GraphicsContext) {
        gctx.begin_pass(self.offscreen_pass, Default::default());
        self.modes.draw(&mut DrawContext {
            gctx,
            level: &self.level,
            actors: &self.actors,
        });
        gctx.end_render_pass();

        gctx.begin_default_pass(Default::default());
        gctx.apply_pipeline(&self.screen_pipeline);
        gctx.apply_bindings(&self.screen_bindings);
        gctx.apply_uniforms(&screen_shader::Uniforms {
            scale: self.window_scale(),
            fade: *self.fade,
        });
        gctx.draw(0, 6, 1);
        gctx.end_render_pass();

        gctx.commit_frame();
    }

    fn key_down_event(
        &mut self,
        gctx: &mut GraphicsContext,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            gctx.request_quit();
        } else {
            self.input.handle_key_down_event(keycode);
        }
    }

    fn key_up_event(&mut self, _gctx: &mut GraphicsContext, keycode: KeyCode, _keymods: KeyMods) {
        self.input.handle_key_up_event(keycode);
    }

    fn resize_event(&mut self, _ctx: &mut GraphicsContext, width: f32, height: f32) {
        self.window_width = width;
        self.window_height = height;
    }

    fn update(&mut self, gctx: &mut GraphicsContext) {
        let current_time = miniquad::date::now();
        self.time_bank += if self.last_time != 0.0 {
            (current_time - self.last_time).min(MAX_FRAME_TIME)
        } else {
            FRAME_TIME
        };
        self.last_time = current_time;
        *self.gctx_ptr = gctx as *mut GraphicsContext;
        while self.time_bank >= FRAME_TIME {
            self.time_bank -= FRAME_TIME;
            let mut dummy_context = Context::from_waker(&self.dummy_waker);
            if let Poll::Ready(()) = self.script.as_mut().poll(&mut dummy_context) {
                gctx.order_quit();
                break;
            }
            self.input.reset_keys_pressed();
        }
        *self.gctx_ptr = std::ptr::null_mut();
    }
}

fn main() {
    miniquad::start(
        miniquad::conf::Conf {
            window_title: "RPG".to_string(),
            window_width: 960,
            window_height: 528,
            ..Default::default()
        },
        |gctx| Box::new(App::new(gctx)),
    );
}
