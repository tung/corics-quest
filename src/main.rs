mod input;
mod ldtk;
mod levels;
mod resources;
mod shaders;

use input::*;
use levels::*;
use resources::*;
use shaders::*;

use miniquad::graphics::{
    Bindings, Buffer, BufferType, GraphicsContext, Pipeline, RenderPass, Texture, TextureFormat,
    TextureParams,
};
use miniquad::{EventHandler, KeyCode, KeyMods};

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 176;

struct App {
    offscreen_pass: RenderPass,
    screen_pipeline: Pipeline,
    screen_bindings: Bindings,
    window_width: f32,
    window_height: f32,
    _res: Resources,
    level: Level,
    camera_x: i32,
    camera_y: i32,
    input: Input,
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

        let level = res.levels.level_by_identifier(gctx, &res, "Start");

        Self {
            offscreen_pass,
            screen_pipeline,
            screen_bindings,
            window_width: SCREEN_WIDTH as f32,
            window_height: SCREEN_HEIGHT as f32,
            _res: res,
            level,
            camera_x: SCREEN_WIDTH as i32 / 2,
            camera_y: SCREEN_HEIGHT as i32 / 2,
            input: Input::new(),
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
        self.level.draw(gctx, self.camera_x, self.camera_y);
        gctx.end_render_pass();

        gctx.begin_default_pass(Default::default());
        gctx.apply_pipeline(&self.screen_pipeline);
        gctx.apply_bindings(&self.screen_bindings);
        gctx.apply_uniforms(&screen_shader::Uniforms {
            scale: self.window_scale(),
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

    fn update(&mut self, _gctx: &mut GraphicsContext) {
        if self.input.is_key_down(GameKey::Up) {
            self.camera_y -= 1;
        }
        if self.input.is_key_down(GameKey::Down) {
            self.camera_y += 1;
        }
        if self.input.is_key_down(GameKey::Left) {
            self.camera_x -= 1;
        }
        if self.input.is_key_down(GameKey::Right) {
            self.camera_x += 1;
        }
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
