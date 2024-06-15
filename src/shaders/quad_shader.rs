use miniquad::{
    BlendFactor, BlendState, BlendValue, BufferLayout, Equation, GlContext, Pipeline,
    PipelineParams, RenderingBackend, ShaderMeta, ShaderSource, UniformBlockLayout, UniformDesc,
    UniformType, VertexAttribute, VertexFormat, VertexStep,
};

const VERTEX: &str = r#"#version 100
precision mediump float;

attribute vec2 pos;
attribute vec2 inst_px_size;
attribute vec2 inst_px_src_pos;
attribute vec2 inst_px_dest_pos;
varying vec2 tex_coord;
uniform vec2 px_src_offset;
uniform vec2 px_dest_offset;
uniform vec2 px_framebuffer_size;
uniform vec2 px_texture_size;

void main() {
    gl_Position = vec4(
        (pos * inst_px_size + inst_px_dest_pos + px_dest_offset)
            * vec2(2.0, -2.0)
            / px_framebuffer_size
            + vec2(-1.0, 1.0),
        0.0,
        1.0
    );
    tex_coord = (pos * inst_px_size + inst_px_src_pos + px_src_offset) / px_texture_size;
}
"#;

const FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec2 tex_coord;
uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, tex_coord);
}
"#;

#[repr(C)]
pub struct Uniforms {
    pub px_src_offset: [f32; 2],
    pub px_dest_offset: [f32; 2],
    pub px_framebuffer_size: [f32; 2],
    pub px_texture_size: [f32; 2],
}

pub fn pipeline(gctx: &mut GlContext) -> Pipeline {
    let shader = gctx
        .new_shader(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("px_src_offset", UniformType::Float2),
                        UniformDesc::new("px_dest_offset", UniformType::Float2),
                        UniformDesc::new("px_framebuffer_size", UniformType::Float2),
                        UniformDesc::new("px_texture_size", UniformType::Float2),
                    ],
                },
            },
        )
        .unwrap();

    gctx.new_pipeline(
        &[
            BufferLayout::default(),
            BufferLayout {
                step_func: VertexStep::PerInstance,
                ..Default::default()
            },
        ],
        &[
            VertexAttribute::with_buffer("pos", VertexFormat::Float2, 0),
            VertexAttribute::with_buffer("inst_px_size", VertexFormat::Float2, 1),
            VertexAttribute::with_buffer("inst_px_src_pos", VertexFormat::Float2, 1),
            VertexAttribute::with_buffer("inst_px_dest_pos", VertexFormat::Float2, 1),
        ],
        shader,
        PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            alpha_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Zero,
                BlendFactor::One,
            )),
            ..Default::default()
        },
    )
}
