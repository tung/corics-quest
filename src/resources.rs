use crate::aseprite;
use crate::levels::*;
use crate::shaders::*;

use miniquad::{
    BufferId, FilterMode, GlContext, MipmapFilterMode, Pipeline, RenderingBackend, TextureId,
    TextureWrap,
};

use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

const SPRITE_SHEETS_BY_PATH: &[(&str, &str)] = &[
    ("basilisk.png", include_str!("../assets/basilisk.json")),
    ("bat.png", include_str!("../assets/bat.json")),
    ("bed.png", include_str!("../assets/bed.json")),
    ("chest.png", include_str!("../assets/chest.json")),
    ("cobra.png", include_str!("../assets/cobra.json")),
    ("coric.png", include_str!("../assets/coric.json")),
    ("dog.png", include_str!("../assets/dog.json")),
    ("dragonfly.png", include_str!("../assets/dragonfly.json")),
    ("ducille.png", include_str!("../assets/ducille.json")),
    ("earth.png", include_str!("../assets/earth.json")),
    (
        "earth-small.png",
        include_str!("../assets/earth-small.json"),
    ),
    ("fang-frog.png", include_str!("../assets/fang-frog.json")),
    ("fire.png", include_str!("../assets/fire.json")),
    ("fire-small.png", include_str!("../assets/fire-small.json")),
    ("ghost.png", include_str!("../assets/ghost.json")),
    ("golem.png", include_str!("../assets/golem.json")),
    ("griffon.png", include_str!("../assets/griffon.json")),
    ("horn-beast.png", include_str!("../assets/horn-beast.json")),
    ("jace.png", include_str!("../assets/jace.json")),
    ("jelly.png", include_str!("../assets/jelly.json")),
    ("julis.png", include_str!("../assets/julis.json")),
    ("leech.png", include_str!("../assets/leech.json")),
    ("lever.png", include_str!("../assets/lever.json")),
    ("matero.png", include_str!("../assets/matero.json")),
    ("minotaur.png", include_str!("../assets/minotaur.json")),
    ("orc.png", include_str!("../assets/orc.json")),
    ("rat.png", include_str!("../assets/rat.json")),
    ("rogue.png", include_str!("../assets/rogue.json")),
    ("scorpion.png", include_str!("../assets/scorpion.json")),
    ("serpent.png", include_str!("../assets/serpent.json")),
    ("shambler.png", include_str!("../assets/shambler.json")),
    ("troll.png", include_str!("../assets/troll.json")),
    ("turtle.png", include_str!("../assets/turtle.json")),
    ("vampire.png", include_str!("../assets/vampire.json")),
    ("warlock.png", include_str!("../assets/warlock.json")),
    ("war-tusk.png", include_str!("../assets/war-tusk.json")),
    ("water.png", include_str!("../assets/water.json")),
    (
        "water-small.png",
        include_str!("../assets/water-small.json"),
    ),
];

const TEXTURES_BY_PATH: &[(&str, &[u8])] = &[
    ("base.png", include_bytes!("../assets/base.png")),
    ("basilisk.png", include_bytes!("../assets/basilisk.png")),
    ("bat.png", include_bytes!("../assets/bat.png")),
    ("bed.png", include_bytes!("../assets/bed.png")),
    ("chest.png", include_bytes!("../assets/chest.png")),
    ("cobra.png", include_bytes!("../assets/cobra.png")),
    ("coric.png", include_bytes!("../assets/coric.png")),
    ("dog.png", include_bytes!("../assets/dog.png")),
    ("dragonfly.png", include_bytes!("../assets/dragonfly.png")),
    ("ducille.png", include_bytes!("../assets/ducille.png")),
    ("earth.png", include_bytes!("../assets/earth.png")),
    (
        "earth-small.png",
        include_bytes!("../assets/earth-small.png"),
    ),
    ("edges.png", include_bytes!("../assets/edges.png")),
    ("fang-frog.png", include_bytes!("../assets/fang-frog.png")),
    ("fire.png", include_bytes!("../assets/fire.png")),
    ("fire-small.png", include_bytes!("../assets/fire-small.png")),
    ("grassdirt.png", include_bytes!("../assets/grassdirt.png")),
    ("ghost.png", include_bytes!("../assets/ghost.png")),
    ("golem.png", include_bytes!("../assets/golem.png")),
    ("griffon.png", include_bytes!("../assets/griffon.png")),
    ("horn-beast.png", include_bytes!("../assets/horn-beast.png")),
    ("jace.png", include_bytes!("../assets/jace.png")),
    ("jelly.png", include_bytes!("../assets/jelly.png")),
    ("julis.png", include_bytes!("../assets/julis.png")),
    ("leech.png", include_bytes!("../assets/leech.png")),
    ("lever.png", include_bytes!("../assets/lever.png")),
    ("matero.png", include_bytes!("../assets/matero.png")),
    ("minotaur.png", include_bytes!("../assets/minotaur.png")),
    ("orc.png", include_bytes!("../assets/orc.png")),
    ("props.png", include_bytes!("../assets/props.png")),
    ("rat.png", include_bytes!("../assets/rat.png")),
    ("rogue.png", include_bytes!("../assets/rogue.png")),
    ("scorpion.png", include_bytes!("../assets/scorpion.png")),
    ("serpent.png", include_bytes!("../assets/serpent.png")),
    ("shambler.png", include_bytes!("../assets/shambler.png")),
    ("troll.png", include_bytes!("../assets/troll.png")),
    ("turtle.png", include_bytes!("../assets/turtle.png")),
    ("vampire.png", include_bytes!("../assets/vampire.png")),
    ("warlock.png", include_bytes!("../assets/warlock.png")),
    ("war-tusk.png", include_bytes!("../assets/war-tusk.png")),
    ("water.png", include_bytes!("../assets/water.png")),
    (
        "water-small.png",
        include_bytes!("../assets/water-small.png"),
    ),
];

pub struct Resources {
    pub quad_vbuf: BufferId,
    pub quad_ibuf: BufferId,
    pub layer_pipeline: Pipeline,
    pub quad_pipeline: Pipeline,
    pub levels: LevelSet,
    pub font: Texture,
    pub window_textures: WindowTextures,
    pub textures_by_path: HashMap<&'static str, Texture>,
    pub sprite_sheets_by_path: HashMap<&'static str, Rc<aseprite::SpriteSheet>>,
}

#[derive(Clone, Copy)]
pub struct Texture {
    pub tex_id: TextureId,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct WindowTextures {
    pub center: Texture,
    pub h_edges: Texture,
    pub v_edges: Texture,
    pub corners: Texture,
    pub px_wid_left: u32,
    pub px_hei_top: u32,
}

impl Resources {
    pub fn new(gctx: &mut GlContext, quad_vbuf: BufferId, quad_ibuf: BufferId) -> Self {
        let layer_pipeline = layer_shader::pipeline(gctx);
        let quad_pipeline = quad_shader::pipeline(gctx);
        let font = texture_from_png_bytes(gctx, &include_bytes!("../assets/hp-100lx-6x8.png")[..]);
        let window_textures = WindowTextures::new(gctx);

        let textures_by_path = TEXTURES_BY_PATH
            .iter()
            .map(|(p, b)| (*p, texture_from_png_bytes(gctx, b)))
            .collect::<HashMap<_, _>>();

        let sprite_sheets_by_path = SPRITE_SHEETS_BY_PATH
            .iter()
            .map(|(p, j)| {
                (
                    *p,
                    Rc::new(miniserde::json::from_str::<aseprite::SpriteSheet>(j).unwrap()),
                )
            })
            .collect::<HashMap<_, _>>();

        Self {
            quad_vbuf,
            quad_ibuf,
            layer_pipeline,
            quad_pipeline,
            levels: LevelSet::new(),
            font,
            window_textures,
            textures_by_path,
            sprite_sheets_by_path,
        }
    }
}

impl WindowTextures {
    fn new(gctx: &mut GlContext) -> Self {
        const PX_WID_LEFT: u32 = 4;
        const PX_HEI_TOP: u32 = 4;

        let center =
            texture_from_png_bytes(gctx, &include_bytes!("../assets/window-center.png")[..]);
        let h_edges =
            texture_from_png_bytes(gctx, &include_bytes!("../assets/window-h-edges.png")[..]);
        let v_edges =
            texture_from_png_bytes(gctx, &include_bytes!("../assets/window-v-edges.png")[..]);
        let corners =
            texture_from_png_bytes(gctx, &include_bytes!("../assets/window-corners.png")[..]);

        assert!(PX_WID_LEFT < v_edges.width);
        assert!(PX_HEI_TOP < h_edges.height);
        assert!(corners.width == v_edges.width);
        assert!(corners.height == h_edges.height);

        gctx.texture_set_wrap(center.tex_id, TextureWrap::Repeat, TextureWrap::Repeat);
        gctx.texture_set_wrap(h_edges.tex_id, TextureWrap::Repeat, TextureWrap::Clamp);
        gctx.texture_set_wrap(v_edges.tex_id, TextureWrap::Clamp, TextureWrap::Repeat);

        Self {
            center,
            h_edges,
            v_edges,
            corners,
            px_wid_left: PX_WID_LEFT,
            px_hei_top: PX_HEI_TOP,
        }
    }
}

pub fn texture_from_png_bytes(gctx: &mut GlContext, png_bytes: &[u8]) -> Texture {
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

    let tex_id = gctx.new_texture_from_rgba8(
        info.width.try_into().expect("width"),
        info.height.try_into().expect("height"),
        &pixels[..],
    );
    gctx.texture_set_filter(tex_id, FilterMode::Nearest, MipmapFilterMode::None);

    Texture {
        tex_id,
        width: info.width,
        height: info.height,
    }
}
