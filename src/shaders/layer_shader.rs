use miniquad::{
    BlendFactor, BlendState, BlendValue, BufferLayout, Equation, GlContext, Pipeline,
    PipelineParams, RenderingBackend, ShaderMeta, ShaderSource, UniformBlockLayout, UniformDesc,
    UniformType, VertexAttribute, VertexFormat,
};

const VERTEX: &str = r#"#version 100
precision mediump float;

attribute vec2 pos;
varying vec2 tile_data_coord;
uniform float px_tile_grid_size;
uniform vec2 c_layer_size;
uniform vec2 px_offset;
uniform vec2 px_framebuffer_size;

void main() {
    gl_Position = vec4(
        (pos * c_layer_size * px_tile_grid_size + px_offset)
            * vec2(2.0, -2.0)
            / px_framebuffer_size
            + vec2(-1.0, 1.0),
        0.0,
        1.0
    );
    tile_data_coord = pos;
}
"#;

const FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec2 tile_data_coord;
uniform vec2 c_layer_size;
uniform vec2 tile_to_tileset_ratio;
uniform sampler2D tile_data;
uniform sampler2D tileset;

void main() {
    vec2 c_base = texture2D(tile_data, tile_data_coord).xy * 255.0;
    vec2 c_offset = fract(tile_data_coord * c_layer_size);
    gl_FragColor = texture2D(tileset, (c_base + c_offset) * tile_to_tileset_ratio);
}
"#;

#[repr(C)]
pub struct Uniforms {
    pub px_tile_grid_size: f32,
    pub c_layer_size: [f32; 2],
    pub px_offset: [f32; 2],
    pub px_framebuffer_size: [f32; 2],
    pub tile_to_tileset_ratio: [f32; 2],
}

pub fn pipeline(gctx: &mut GlContext) -> Pipeline {
    let shader = gctx
        .new_shader(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            ShaderMeta {
                images: vec!["tile_data".to_string(), "tileset".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("px_tile_grid_size", UniformType::Float1),
                        UniformDesc::new("c_layer_size", UniformType::Float2),
                        UniformDesc::new("px_offset", UniformType::Float2),
                        UniformDesc::new("px_framebuffer_size", UniformType::Float2),
                        UniformDesc::new("tile_to_tileset_ratio", UniformType::Float2),
                    ],
                },
            },
        )
        .unwrap();

    gctx.new_pipeline(
        &[BufferLayout::default()],
        &[VertexAttribute::new("pos", VertexFormat::Float2)],
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
