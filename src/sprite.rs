use crate::direction::*;
use crate::resources::*;
use crate::{aseprite, quad_shader, SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::graphics::{Bindings, Buffer, BufferType, GraphicsContext, Pipeline, Texture};

use std::rc::Rc;

pub struct Sprite {
    json: Rc<aseprite::SpriteSheet>,
    tag: usize,
    frame: usize,
    t: f32,
    next_frame_forward: bool,
    quad_pipeline: Pipeline,
    bindings: Bindings,
}

impl Sprite {
    pub fn new(gctx: &mut GraphicsContext, res: &Resources, texture_path: &str) -> Self {
        let json = &res.sprite_sheets_by_path[texture_path];

        // At least one animation "tag" must be defined.
        assert!(!json.meta.frame_tags.is_empty());

        let inst_buf = Buffer::immutable(
            gctx,
            BufferType::VertexBuffer,
            &[
                [json.frames[0].frame.w as f32, json.frames[0].frame.h as f32],
                [0.0, 0.0],
                [0.0, 0.0],
            ],
        );
        let texture = res.textures_by_path[json.meta.image.as_str()];

        Self {
            json: Rc::clone(json),
            tag: 0,
            frame: 0,
            t: 0.0,
            next_frame_forward: true,
            quad_pipeline: res.quad_pipeline,
            bindings: Bindings {
                vertex_buffers: vec![res.quad_vbuf, inst_buf],
                index_buffer: res.quad_ibuf,
                images: vec![texture],
            },
        }
    }

    pub fn animate(&mut self) {
        let duration = self.json.frames[self.frame].duration as f32;
        self.t += 1000.0 / 60.0;
        if self.t >= duration {
            self.t -= duration;

            let tag_data = &self.json.meta.frame_tags[self.tag];
            if self.next_frame_forward {
                if self.frame < tag_data.to {
                    self.frame += 1;
                } else {
                    match &tag_data.direction[..] {
                        "forward" => self.frame = tag_data.from,
                        "pingpong" | "pingpong_reverse" => {
                            self.frame = tag_data.from.max(self.frame.saturating_sub(1));
                            self.next_frame_forward = false;
                        }
                        _ => unimplemented!(),
                    }
                }
            } else if self.frame > tag_data.from {
                self.frame -= 1;
            } else {
                match &tag_data.direction[..] {
                    "reverse" => self.frame = tag_data.to,
                    "pingpong" | "pingpong_reverse" => {
                        self.frame = tag_data.to.min(self.frame.saturating_add(1));
                        self.next_frame_forward = true;
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }

    pub fn draw(&self, gctx: &mut GraphicsContext, x: i32, y: i32) {
        let aseprite::Rect {
            x: src_x, y: src_y, ..
        } = self.json.frames[self.frame].frame;
        let Texture {
            width: px_texture_width,
            height: px_texture_height,
            ..
        } = self.bindings.images[0];

        gctx.apply_pipeline(&self.quad_pipeline);
        gctx.apply_bindings(&self.bindings);
        gctx.apply_uniforms(&quad_shader::Uniforms {
            px_src_offset: [src_x as f32, src_y as f32],
            px_dest_offset: [x as f32, y as f32],
            px_framebuffer_size: [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            px_texture_size: [px_texture_width as f32, px_texture_height as f32],
        });
        gctx.draw(0, 6, 1);
    }

    pub fn start_animation(&mut self, tag: &str) {
        let (tag_index, tag_data) = self
            .json
            .meta
            .frame_tags
            .iter()
            .enumerate()
            .find(|(_, t)| t.name == tag)
            .expect("tag with matching name");
        assert!(matches!(&tag_data.direction[..], "forward" | "pingpong"));
        if tag_index != self.tag {
            self.tag = tag_index;
            self.frame = tag_data.from;
            self.t = 0.0;
            self.next_frame_forward = true;
        }
    }

    pub fn start_walk_animation(&mut self, dir: Direction) {
        self.start_animation(match dir {
            Direction::North => "walk_n",
            Direction::East => "walk_e",
            Direction::South => "walk_s",
            Direction::West => "walk_w",
        });
    }

    pub fn stop_walk_animation(&mut self) {
        let face_tag = match &self.json.meta.frame_tags[self.tag].name[..] {
            "walk_n" => Some("face_n"),
            "walk_e" => Some("face_e"),
            "walk_s" => Some("face_s"),
            "walk_w" => Some("face_w"),
            _ => None,
        };
        if let Some(face_tag) = face_tag {
            self.start_animation(face_tag);
        }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        self.bindings.vertex_buffers[1].delete();
    }
}
