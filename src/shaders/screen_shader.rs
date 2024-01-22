use miniquad::graphics::{
    BufferLayout, GraphicsContext, Pipeline, Shader, ShaderMeta, UniformBlockLayout, UniformDesc,
    UniformType, VertexAttribute, VertexFormat,
};

const VERTEX: &str = r#"#version 100
precision mediump float;

attribute vec2 pos;
varying vec2 tex_coord;
uniform vec2 scale;

void main() {
    gl_Position = vec4((pos * 2.0 - 1.0) * scale, 0.0, 1.0);
    tex_coord = pos;
}
"#;

const FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec2 tex_coord;
uniform sampler2D offscreen_texture;

void main() {
    gl_FragColor = texture2D(offscreen_texture, tex_coord);
}
"#;

#[repr(C)]
pub struct Uniforms {
    pub scale: [f32; 2],
}

pub fn pipeline(gctx: &mut GraphicsContext) -> Pipeline {
    let shader = Shader::new(
        gctx,
        VERTEX,
        FRAGMENT,
        ShaderMeta {
            images: vec!["offscreen_texture".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("scale", UniformType::Float2)],
            },
        },
    )
    .unwrap();

    Pipeline::new(
        gctx,
        &[BufferLayout::default()],
        &[VertexAttribute::new("pos", VertexFormat::Float2)],
        shader,
    )
}
