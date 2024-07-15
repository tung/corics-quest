mod actor;
mod aseprite;
mod async_utils;
mod audio;
mod contexts;
mod direction;
mod enemy;
mod fade;
mod input;
mod ldtk;
mod levels;
mod meter;
mod modes;
mod progress;
mod random;
mod resources;
mod saved_options;
mod script;
mod shaders;
mod sprite;
mod text;
mod window;

use actor::*;
use async_utils::*;
use audio::*;
use contexts::*;
use fade::*;
use input::*;
use levels::*;
use modes::*;
use resources::*;
use saved_options::*;
use shaders::*;

use miniquad::{
    Bindings, BufferSource, BufferType, BufferUsage, EventHandler, GlContext, KeyCode, KeyMods,
    Pipeline, RenderPass, RenderingBackend, TextureFormat, TextureParams, UniformsSource,
};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 176;
const FRAME_TIME: f64 = 1.0 / 60.0;
const MAX_FRAME_TIME: f64 = FRAME_TIME * 4.0;

struct App {
    script: Option<Pin<Box<dyn Future<Output = ()>>>>,
    dummy_waker: Waker,
    last_time: f64,
    time_bank: f64,
    offscreen_pass: RenderPass,
    screen_pipeline: Pipeline,
    screen_bindings: Bindings,
    window_width: f32,
    window_height: f32,
    input: SharedMut<Input>,
    audio: SharedMut<Audio>,
    modes: SharedMut<ModeStack>,
    level: SharedMut<Level>,
    actors: SharedMut<Vec<Actor>>,
    fade: SharedMut<Fade>,
}

impl App {
    fn new() -> Self {
        let gctx = get_gctx();

        let offscreen_tex_id = gctx.new_render_texture(TextureParams {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            format: TextureFormat::RGBA8,
            ..Default::default()
        });
        let offscreen_pass = gctx.new_render_pass(offscreen_tex_id, None);

        let screen_pipeline = screen_shader::pipeline(gctx);

        let quad_vbuf = gctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&[[0.0f32, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]]),
        );
        let quad_ibuf = gctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&[0u16, 2, 1, 1, 2, 3]),
        );
        let screen_bindings = Bindings {
            vertex_buffers: vec![quad_vbuf],
            index_buffer: quad_ibuf,
            images: vec![offscreen_tex_id],
        };

        let res = Resources::new(gctx, quad_vbuf, quad_ibuf);

        // Load options on a best-effort basis.
        let opts = SavedOptions::load().unwrap_or_else(|_| SavedOptions::new());

        let input = SharedMut::new(Input::new());
        let audio = SharedMut::new(Audio::new(opts.music_volume, opts.sound_volume));
        let modes = SharedMut::new(ModeStack::new());
        let (level, actors) = {
            let (level, mut actors) = res.levels.level_by_identifier(gctx, &res, "Start");
            let mut player = Actor::new(gctx, &res, ActorType::Player, 6, 3, "coric.png");
            player.start_animation("face_s");
            actors.insert(0, player);
            (SharedMut::new(level), SharedMut::new(actors))
        };
        let fade = SharedMut::new(Fade::new());
        let sctx = ScriptContext::new(res, &input, &audio, &modes, &level, &actors, &fade);

        Self {
            script: Some(Box::pin(script::script_main(sctx))),
            dummy_waker: async_utils::new_dummy_waker(),
            last_time: 0.0,
            time_bank: 0.0,
            offscreen_pass,
            screen_pipeline,
            screen_bindings,
            window_width: SCREEN_WIDTH as f32,
            window_height: SCREEN_HEIGHT as f32,
            input,
            audio,
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
    fn draw(&mut self) {
        let gctx = get_gctx();

        gctx.begin_pass(Some(self.offscreen_pass), Default::default());
        self.modes.draw(&mut DrawContext {
            gctx,
            level: &self.level,
            actors: &self.actors,
        });
        gctx.end_render_pass();

        gctx.begin_default_pass(Default::default());
        gctx.apply_pipeline(&self.screen_pipeline);
        gctx.apply_bindings(&self.screen_bindings);
        gctx.apply_uniforms(UniformsSource::table(&screen_shader::Uniforms {
            scale: self.window_scale(),
            fade: (&*self.fade).into(),
        }));
        gctx.draw(0, 6, 1);
        gctx.end_render_pass();

        gctx.commit_frame();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input.handle_key_down_event(keycode);
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.input.handle_key_up_event(keycode);
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.window_width = width;
        self.window_height = height;
    }

    fn update(&mut self) {
        let current_time = miniquad::date::now();
        self.time_bank += if self.last_time != 0.0 {
            (current_time - self.last_time).min(MAX_FRAME_TIME)
        } else {
            FRAME_TIME
        };
        self.last_time = current_time;
        while self.time_bank >= FRAME_TIME {
            self.time_bank -= FRAME_TIME;
            self.audio.adjust_music_volume_scripted();
            if let Some(script) = &mut self.script {
                let mut dummy_context = Context::from_waker(&self.dummy_waker);
                if let Poll::Ready(()) = script.as_mut().poll(&mut dummy_context) {
                    miniquad::window::order_quit();
                    self.script = None;
                    break;
                }
            }
            self.input.reset_keys_pressed();
        }
    }
}

fn get_gctx() -> &'static mut GlContext {
    static mut GCTX: Option<GlContext> = None;
    // SAFETY: All graphics operations are single-threaded.
    unsafe { GCTX.get_or_insert_with(GlContext::new) }
}

fn main() {
    miniquad::start(
        miniquad::conf::Conf {
            window_title: "RPG".to_string(),
            window_width: 960,
            window_height: 528,
            ..Default::default()
        },
        || Box::new(App::new()),
    );
}
