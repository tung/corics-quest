use crate::resources::*;
use crate::shaders::quad_shader;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::graphics::{
    Bindings, Buffer, BufferType, GraphicsContext, Pipeline, Texture, TextureWrap,
};

pub struct Meter {
    offset: [f32; 2],
    width: f32,
    max_value: i32,
    bindings: Bindings,
    quad_pipeline: Pipeline,
}

impl Meter {
    pub fn new(
        gctx: &mut GraphicsContext,
        res: &Resources,
        x: i32,
        y: i32,
        width: i32,
        color: [u8; 3],
        max_value: i32,
    ) -> Self {
        let inst_buf = Buffer::stream(
            gctx,
            BufferType::VertexBuffer,
            5 * std::mem::size_of::<[[f32; 2]; 3]>(),
        );

        #[rustfmt::skip]
        let texture_data = [
            224, 224, 224, 255,
            224, 224, 224, 255,
            color[0], color[1], color[2], 255,
            color[0], color[1], color[2], 255,
        ];
        let texture = Texture::from_rgba8(gctx, 1, 4, &texture_data[..]);
        texture.set_wrap_xy(gctx, TextureWrap::Repeat, TextureWrap::Clamp);

        let mut meter = Self {
            offset: [x as f32, y as f32],
            width: width as f32,
            max_value,
            bindings: Bindings {
                vertex_buffers: vec![res.quad_vbuf, inst_buf],
                index_buffer: res.quad_ibuf,
                images: vec![texture],
            },
            quad_pipeline: res.quad_pipeline,
        };
        meter.set_value_and_max(gctx, max_value, max_value);
        meter
    }

    pub fn draw(&self, gctx: &mut GraphicsContext) {
        gctx.apply_pipeline(&self.quad_pipeline);
        gctx.apply_bindings(&self.bindings);
        gctx.apply_uniforms(&quad_shader::Uniforms {
            px_src_offset: [0.0, 0.0],
            px_dest_offset: self.offset,
            px_framebuffer_size: [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            px_texture_size: [1.0, 4.0],
        });
        if self.width >= 3.0 {
            gctx.draw(0, 6, 5);
        } else if self.width >= 2.0 {
            gctx.draw(0, 6, 2);
        } else if self.width >= 1.0 {
            gctx.draw(0, 6, 1);
        }
    }

    pub fn set_value(&mut self, gctx: &mut GraphicsContext, value: i32) {
        let max_value = self.max_value;
        self.set_value_and_max(gctx, value, max_value);
    }

    pub fn set_value_and_max(&mut self, gctx: &mut GraphicsContext, value: i32, max_value: i32) {
        self.max_value = max_value.max(1);
        let value = value.max(0).min(self.max_value) as f32;
        let inst_data: [[[f32; 2]; 3]; 5] = [
            // left edge
            [[1.0, 2.0], [0.0, 0.0], [0.0, 1.0]],
            // right edge
            [[1.0, 2.0], [0.0, 0.0], [self.width - 1.0, 1.0]],
            // top edge
            [[self.width - 2.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            // bottom edge
            [[self.width - 2.0, 1.0], [0.0, 0.0], [1.0, 3.0]],
            // inner bar
            [
                [
                    value / self.max_value as f32 * (self.width - 2.0).max(0.0),
                    2.0,
                ],
                [0.0, 2.0],
                [1.0, 1.0],
            ],
        ];
        self.bindings.vertex_buffers[1].update(gctx, &inst_data[..]);
    }
}

impl Drop for Meter {
    fn drop(&mut self) {
        self.bindings.vertex_buffers[1].delete();
        self.bindings.images[0].delete();
    }
}
