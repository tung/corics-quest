use miniquad::{
    BufferLayout, GlContext, Pipeline, RenderingBackend, ShaderMeta, ShaderSource,
    UniformBlockLayout, UniformDesc, UniformType, VertexAttribute, VertexFormat,
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
uniform vec4 fade;
uniform sampler2D offscreen_texture;

void main() {
    gl_FragColor = vec4(
        vec3(1.0 - fade.a)
            * texture2D(offscreen_texture, tex_coord).rgb
            + vec3(fade.a)
            * fade.rgb,
        1.0
    );
}
"#;

#[repr(C)]
pub struct Uniforms {
    pub scale: [f32; 2],
    pub fade: [f32; 4],
}

pub fn pipeline(gctx: &mut GlContext) -> Pipeline {
    let shader = gctx
        .new_shader(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            ShaderMeta {
                images: vec!["offscreen_texture".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("scale", UniformType::Float2),
                        UniformDesc::new("fade", UniformType::Float4),
                    ],
                },
            },
        )
        .unwrap();

    gctx.new_pipeline(
        &[BufferLayout::default()],
        &[VertexAttribute::new("pos", VertexFormat::Float2)],
        shader,
        Default::default(),
    )
}
