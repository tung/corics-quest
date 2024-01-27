use crate::resources::*;
use crate::shaders::quad_shader;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::graphics::{Bindings, Buffer, BufferType, GraphicsContext, Pipeline, Texture};

pub struct Window {
    x: f32,
    y: f32,
    px_wid_left: u32,
    px_wid_right: u32,
    px_hei_top: u32,
    px_hei_bottom: u32,
    center: WindowPart<1>,
    h_edges: WindowPart<2>,
    v_edges: WindowPart<2>,
    corners: WindowPart<4>,
}

struct WindowPart<const N: usize> {
    inst_data: [[[f32; 2]; 3]; N],
    len: usize,
    bindings: Bindings,
    quad_pipeline: Pipeline,
}

impl Window {
    pub fn new(
        gctx: &mut GraphicsContext,
        res: &Resources,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        let center = WindowPart::new(gctx, res.window_textures.center, res);
        let h_edges = WindowPart::new(gctx, res.window_textures.h_edges, res);
        let v_edges = WindowPart::new(gctx, res.window_textures.v_edges, res);
        let corners = WindowPart::new(gctx, res.window_textures.corners, res);

        let mut window = Self {
            x: x as f32,
            y: y as f32,
            px_wid_left: res.window_textures.px_wid_left,
            px_wid_right: res.window_textures.v_edges.width - res.window_textures.px_wid_left,
            px_hei_top: res.window_textures.px_hei_top,
            px_hei_bottom: res.window_textures.h_edges.height - res.window_textures.px_hei_top,
            center,
            h_edges,
            v_edges,
            corners,
        };
        window.resize(gctx, width, height);
        window
    }

    pub fn draw(&self, gctx: &mut GraphicsContext) {
        self.center.draw(
            gctx,
            self.x + self.px_wid_left as f32,
            self.y + self.px_hei_top as f32,
        );
        self.v_edges
            .draw(gctx, self.x, self.y + self.px_hei_top as f32);
        self.h_edges
            .draw(gctx, self.x + self.px_wid_left as f32, self.y);
        self.corners.draw(gctx, self.x, self.y);
    }

    pub fn resize(&mut self, gctx: &mut GraphicsContext, width: i32, height: i32) {
        let width = width.max(0) as f32;
        let height = height.max(0) as f32;
        let widths = [
            width.min(self.px_wid_left as f32),
            (width - (self.px_wid_left + self.px_wid_right) as f32).max(0.0),
            (width - self.px_wid_left as f32)
                .max(0.0)
                .min(self.px_wid_right as f32),
        ];
        let heights = [
            height.min(self.px_hei_top as f32),
            (height - (self.px_hei_top + self.px_hei_bottom) as f32).max(0.0),
            (height - self.px_hei_top as f32)
                .max(0.0)
                .min(self.px_hei_bottom as f32),
        ];

        // update center size
        if widths[1] > 0.0 && heights[1] > 0.0 {
            let new_inst_data = [[[widths[1], heights[1]], [0.0, 0.0], [0.0, 0.0]]];
            let new_len = 1;
            self.center.len = new_len;
            if self.center.inst_data[..new_len] != new_inst_data[..new_len] {
                self.center.bindings.vertex_buffers[1].update(gctx, &new_inst_data[..new_len]);
                self.center.inst_data = new_inst_data;
            }
        } else {
            self.center.len = 0;
        }

        // update v_edges sizes
        if widths[0] > 0.0 && heights[1] > 0.0 {
            let new_inst_data = [
                [[widths[0], heights[1]], [0.0, 0.0], [0.0, 0.0]],
                [
                    [widths[2], heights[1]],
                    [self.px_wid_left as f32, 0.0],
                    [widths[0] + widths[1], 0.0],
                ],
            ];
            let new_len = if widths[2] > 0.0 { 2 } else { 1 };
            self.v_edges.len = new_len;
            if self.v_edges.inst_data[..new_len] != new_inst_data[..new_len] {
                self.v_edges.bindings.vertex_buffers[1].update(gctx, &new_inst_data[..new_len]);
                self.v_edges.inst_data = new_inst_data;
            }
        } else {
            self.v_edges.len = 0;
        }

        // update h_edges sizes
        if widths[1] > 0.0 && heights[0] > 0.0 {
            let new_inst_data = [
                [[widths[1], heights[0]], [0.0, 0.0], [0.0, 0.0]],
                [
                    [widths[1], heights[2]],
                    [0.0, self.px_hei_top as f32],
                    [0.0, heights[0] + heights[1]],
                ],
            ];
            let new_len = if heights[2] > 0.0 { 2 } else { 1 };
            self.h_edges.len = new_len;
            if self.h_edges.inst_data[..new_len] != new_inst_data[..new_len] {
                self.h_edges.bindings.vertex_buffers[1].update(gctx, &new_inst_data[..new_len]);
                self.h_edges.inst_data = new_inst_data;
            }
        } else {
            self.h_edges.len = 0;
        }

        // update corner sizes
        if widths[0] > 0.0 || heights[0] > 0.0 {
            // start with top-left corner
            let mut new_inst_data = [
                [[widths[0], heights[0]], [0.0, 0.0], [0.0, 0.0]],
                [[0.0; 2]; 3],
                [[0.0; 2]; 3],
                [[0.0; 2]; 3],
            ];
            let mut new_len = 1;
            // top-right corner
            if widths[2] > 0.0 && heights[0] > 0.0 {
                new_inst_data[new_len] = [
                    [widths[2], heights[0]],
                    [self.px_wid_left as f32, 0.0],
                    [widths[0] + widths[1], 0.0],
                ];
                new_len += 1;
            }
            // bottom-left corner
            if widths[0] > 0.0 && heights[2] > 0.0 {
                new_inst_data[new_len] = [
                    [widths[0], heights[2]],
                    [0.0, self.px_hei_top as f32],
                    [0.0, heights[0] + heights[1]],
                ];
                new_len += 1;
            }
            // bottom-right corner
            if widths[2] > 0.0 && heights[2] > 0.0 {
                new_inst_data[new_len] = [
                    [widths[2], heights[2]],
                    [self.px_wid_left as f32, self.px_hei_top as f32],
                    [widths[0] + widths[1], heights[0] + heights[1]],
                ];
                new_len += 1;
            }
            self.corners.len = new_len;
            if self.corners.inst_data[..new_len] != new_inst_data[..new_len] {
                self.corners.bindings.vertex_buffers[1].update(gctx, &new_inst_data[..new_len]);
                self.corners.inst_data = new_inst_data;
            }
        } else {
            self.corners.len = 0;
        }
    }
}

impl<const N: usize> WindowPart<N> {
    fn new(gctx: &mut GraphicsContext, texture: Texture, res: &Resources) -> Self {
        let inst_buf = Buffer::stream(
            gctx,
            BufferType::VertexBuffer,
            N * std::mem::size_of::<[[f32; 2]; 3]>(),
        );

        Self {
            inst_data: [[[0.0f32; 2]; 3]; N],
            len: 0,
            bindings: Bindings {
                vertex_buffers: vec![res.quad_vbuf, inst_buf],
                index_buffer: res.quad_ibuf,
                images: vec![texture],
            },
            quad_pipeline: res.quad_pipeline,
        }
    }

    fn draw(&self, gctx: &mut GraphicsContext, x: f32, y: f32) {
        if self.len == 0 {
            return;
        }

        gctx.apply_pipeline(&self.quad_pipeline);
        gctx.apply_bindings(&self.bindings);
        gctx.apply_uniforms(&quad_shader::Uniforms {
            px_src_offset: [0.0, 0.0],
            px_dest_offset: [x, y],
            px_framebuffer_size: [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            px_texture_size: [
                self.bindings.images[0].width as f32,
                self.bindings.images[0].height as f32,
            ],
        });
        gctx.draw(0, 6, self.len as i32);
    }
}
