use crate::levels::*;
use crate::shaders::*;

use miniquad::graphics::{Buffer, FilterMode, GraphicsContext, Pipeline, Texture};

use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

const TEXTURES_BY_PATH: &[(&str, &[u8])] = &[
    ("base.png", include_bytes!("../assets/base.png")),
    ("edges.png", include_bytes!("../assets/edges.png")),
    ("grassdirt.png", include_bytes!("../assets/grassdirt.png")),
    ("props.png", include_bytes!("../assets/props.png")),
];

#[derive(Clone)]
pub struct Resources {
    pub quad_vbuf: Buffer,
    pub quad_ibuf: Buffer,
    pub layer_pipeline: Pipeline,
    pub levels: Rc<LevelSet>,
    pub textures_by_path: Rc<HashMap<&'static str, Texture>>,
}

impl Resources {
    pub fn new(gctx: &mut GraphicsContext, quad_vbuf: Buffer, quad_ibuf: Buffer) -> Self {
        let layer_pipeline = layer_shader::pipeline(gctx);

        let textures_by_path = TEXTURES_BY_PATH
            .iter()
            .map(|(p, b)| (*p, texture_from_png_bytes(gctx, b)))
            .collect::<HashMap<_, _>>();

        Self {
            quad_vbuf,
            quad_ibuf,
            layer_pipeline,
            levels: Rc::new(LevelSet::new()),
            textures_by_path: Rc::new(textures_by_path),
        }
    }
}

pub fn texture_from_png_bytes(gctx: &mut GraphicsContext, png_bytes: &[u8]) -> Texture {
    let mut decoder = png::Decoder::new(Cursor::new(png_bytes));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().expect("reader");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("PNG frame");

    let pixels = match info.color_type {
        png::ColorType::Rgb => {
            let mut vec = Vec::with_capacity(buf.len() / 3 * 4);
            for rgb in buf.chunks_exact(3) {
                vec.extend(rgb);
                vec.push(255);
            }
            vec
        }
        png::ColorType::Rgba => buf,
        png::ColorType::Grayscale => {
            let mut vec = Vec::with_capacity(buf.len() * 4);
            for g in buf {
                vec.extend([g, g, g, 255].iter().copied());
            }
            vec
        }
        png::ColorType::GrayscaleAlpha => {
            let mut vec = Vec::with_capacity(buf.len() * 2);
            for ga in buf.chunks_exact(2) {
                let (g, a) = (ga[0], ga[1]);
                vec.extend([g, g, g, a].iter().copied());
            }
            vec
        }
        _ => unreachable!("color type"),
    };

    let texture = Texture::from_rgba8(
        gctx,
        info.width.try_into().expect("width"),
        info.height.try_into().expect("height"),
        &pixels[..],
    );
    texture.set_filter(gctx, FilterMode::Nearest);
    texture
}
