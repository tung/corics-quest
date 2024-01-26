use crate::direction::*;
use crate::resources::*;
use crate::{layer_shader, ldtk, SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::graphics::{Bindings, FilterMode, GraphicsContext, Pipeline, Texture};
use miniserde::json;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub const TILE_SIZE: i32 = 16;

struct Layer {
    c_wid: u16,
    c_hei: u16,
    tileset: Rc<Tileset>,
    tile_data: Vec<u8>,
    layer_pipeline: Pipeline,
    bindings: Bindings,
}

pub struct Level {
    pub identifier: String,
    pub px_world_x: i32,
    pub px_world_y: i32,
    pub px_wid: i32,
    pub px_hei: i32,
    layers: Vec<Layer>,
}

pub struct LevelSet {
    p: ldtk::Project,
    tilesets: TilesetLoader,
    edge_blocked_enum_uid: i64,
    levels_by_identifier: HashMap<String, usize>,
}

struct Tileset {
    texture: Texture,
    tile_grid_size: u16,
    c_wid: usize,
    edges_blocked: Option<Vec<u8>>,
}

struct TilesetLoader(RefCell<HashMap<i64, Weak<Tileset>>>);

impl Layer {
    fn new(
        gctx: &mut GraphicsContext,
        res: &Resources,
        tilesets: &TilesetLoader,
        tileset_defs_json: &[ldtk::TilesetDefinition],
        edge_blocked_enum_uid: i64,
        layer_json: &ldtk::LayerInstance,
    ) -> Option<Self> {
        let c_wid: u16 = layer_json.c_wid.try_into().expect("c_wid as u16");
        let c_hei: u16 = layer_json.c_hei.try_into().expect("c_hei as u16");

        let uid = layer_json
            .override_tileset_uid
            .or(layer_json.tileset_def_uid)?;
        let tileset = tilesets.get_tileset(res, tileset_defs_json, edge_blocked_enum_uid, uid)?;

        let tiles_json = if !layer_json.grid_tiles.is_empty() {
            &layer_json.grid_tiles[..]
        } else if !layer_json.auto_layer_tiles.is_empty() {
            &layer_json.auto_layer_tiles[..]
        } else {
            return None;
        };

        let size = 4 * c_wid as usize * c_hei as usize;
        let mut tile_data = vec![0u8; size];
        for tile in tiles_json {
            let c_src_x = tile.src[0] as u16 / tileset.tile_grid_size;
            let c_src_y = tile.src[1] as u16 / tileset.tile_grid_size;
            let c_dest_x = tile.px[0] as usize / tileset.tile_grid_size as usize;
            let c_dest_y = tile.px[1] as usize / tileset.tile_grid_size as usize;
            let i = 4 * (c_dest_y * c_wid as usize + c_dest_x);
            tile_data[i] = c_src_x.try_into().expect("c_src_x as u8");
            tile_data[i + 1] = c_src_y.try_into().expect("c_src_y as u8");
        }

        let tile_data_texture = Texture::from_rgba8(gctx, c_wid, c_hei, &tile_data[..]);
        tile_data_texture.set_filter(gctx, FilterMode::Nearest);

        let bindings = Bindings {
            vertex_buffers: vec![res.quad_vbuf],
            index_buffer: res.quad_ibuf,
            images: vec![tile_data_texture, tileset.texture],
        };

        Some(Self {
            c_wid,
            c_hei,
            tileset,
            tile_data,
            layer_pipeline: res.layer_pipeline,
            bindings,
        })
    }

    fn draw(&self, gctx: &mut GraphicsContext, camera_x: f32, camera_y: f32) {
        gctx.apply_pipeline(&self.layer_pipeline);
        gctx.apply_bindings(&self.bindings);
        gctx.apply_uniforms(&layer_shader::Uniforms {
            px_tile_grid_size: self.tileset.tile_grid_size as f32,
            c_layer_size: [self.c_wid as f32, self.c_hei as f32],
            px_offset: [
                (SCREEN_WIDTH / 2) as f32 - camera_x,
                (SCREEN_HEIGHT / 2) as f32 - camera_y,
            ],
            px_framebuffer_size: [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
            tile_to_tileset_ratio: [
                self.tileset.tile_grid_size as f32 / self.tileset.texture.width as f32,
                self.tileset.tile_grid_size as f32 / self.tileset.texture.height as f32,
            ],
        });
        gctx.draw(0, 6, 1);
    }

    fn is_edge_blocked(&self, tile_x: i32, tile_y: i32, dir: Direction) -> bool {
        let forward_blocked = if tile_x >= 0
            && tile_x < self.c_wid as i32
            && tile_y >= 0
            && tile_y < self.c_hei as i32
        {
            let offset = 4 * (tile_y as usize * self.c_wid as usize + tile_x as usize);
            self.tileset
                .is_edge_blocked(self.tile_data[offset], self.tile_data[offset + 1], dir)
        } else {
            false
        };

        let tile_x2 = tile_x + dir.dx();
        let tile_y2 = tile_y + dir.dy();
        let backward_blocked = if tile_x2 >= 0
            && tile_x2 < self.c_wid as i32
            && tile_y2 >= 0
            && tile_y2 < self.c_hei as i32
        {
            let offset = 4 * (tile_y2 as usize * self.c_wid as usize + tile_x2 as usize);
            self.tileset.is_edge_blocked(
                self.tile_data[offset],
                self.tile_data[offset + 1],
                dir.reverse(),
            )
        } else {
            false
        };

        forward_blocked || backward_blocked
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        // drop tile data texture
        self.bindings.images[0].delete();
    }
}

impl Level {
    fn new(
        gctx: &mut GraphicsContext,
        res: &Resources,
        tilesets: &TilesetLoader,
        tileset_defs_json: &[ldtk::TilesetDefinition],
        edge_blocked_enum_uid: i64,
        level_json: &ldtk::Level,
    ) -> Self {
        let mut layers = Vec::new();
        for layer_json in level_json
            .layer_instances
            .as_ref()
            .expect("levels saved internally")
            .iter()
        {
            if let Some(layer) = Layer::new(
                gctx,
                res,
                tilesets,
                tileset_defs_json,
                edge_blocked_enum_uid,
                layer_json,
            ) {
                layers.push(layer);
            }
        }

        Self {
            identifier: level_json.identifier.clone(),
            px_world_x: level_json.world_x.try_into().expect("world_x as i32"),
            px_world_y: level_json.world_y.try_into().expect("world_y as i32"),
            px_wid: level_json.px_wid.try_into().expect("px_wid as i32"),
            px_hei: level_json.px_hei.try_into().expect("px_hei as i32"),
            layers,
        }
    }

    pub fn draw(&self, gctx: &mut GraphicsContext, camera_x: i32, camera_y: i32) {
        let camera_x = camera_x as f32;
        let camera_y = camera_y as f32;
        for layer in self.layers.iter().rev() {
            layer.draw(gctx, camera_x, camera_y);
        }
    }

    pub fn is_edge_blocked(&self, tile_x: i32, tile_y: i32, dir: Direction) -> bool {
        self.layers
            .iter()
            .any(|l| l.is_edge_blocked(tile_x, tile_y, dir))
    }
}

impl LevelSet {
    pub fn new() -> Self {
        let p: ldtk::Project =
            json::from_str(include_str!("../assets/project.ldtk")).expect("LDtk project as JSON");

        let tilesets = TilesetLoader::new(&p.defs.tilesets[..]);

        let edge_blocked_enum_uid = p
            .defs
            .enums
            .iter()
            .find(|e| e.identifier == "EdgeBlocked")
            .expect("EdgeBlocked enum")
            .uid;

        let mut levels_by_identifier = HashMap::new();
        for (i, level_json) in p.levels.iter().enumerate() {
            levels_by_identifier.insert(level_json.identifier.clone(), i);
        }

        Self {
            p,
            tilesets,
            edge_blocked_enum_uid,
            levels_by_identifier,
        }
    }

    pub fn level_by_identifier(
        &self,
        gctx: &mut GraphicsContext,
        res: &Resources,
        identifier: &str,
    ) -> Level {
        let level_index = self.levels_by_identifier[identifier];
        Level::new(
            gctx,
            res,
            &self.tilesets,
            &self.p.defs.tilesets[..],
            self.edge_blocked_enum_uid,
            &self.p.levels[level_index],
        )
    }
}

impl Default for LevelSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Tileset {
    fn new(
        res: &Resources,
        edge_blocked_enum_uid: i64,
        tileset_json: &ldtk::TilesetDefinition,
    ) -> Option<Self> {
        let c_wid: usize = tileset_json.c_wid.try_into().expect("c_wid as usize");
        let c_hei: usize = tileset_json.c_hei.try_into().expect("c_hei as usize");

        let edges_blocked = if tileset_json.tags_source_enum_uid == Some(edge_blocked_enum_uid) {
            // Each tile gets 4 edge block bits ordered NESW.
            let mut edges_blocked = vec![0u8; (c_wid * c_hei + 1) / 2];
            for enum_tag_json in &tileset_json.enum_tags {
                let dir_bit = match &enum_tag_json.enum_value_id[..] {
                    "North" => 0,
                    "East" => 1,
                    "South" => 2,
                    "West" => 3,
                    _ => panic!("unknown enum_value_id"),
                };

                for tile_id in &enum_tag_json.tile_ids {
                    let tile_id: usize = (*tile_id).try_into().expect("tile_id as usize");
                    let dest_bit = dir_bit + 4 * (tile_id & 1);
                    let dest_byte = tile_id / 2;
                    edges_blocked[dest_byte] |= 1 << dest_bit;
                }
            }
            Some(edges_blocked)
        } else {
            None
        };

        Some(Self {
            texture: res.textures_by_path[tileset_json.rel_path.as_ref()?.as_str()],
            tile_grid_size: tileset_json
                .tile_grid_size
                .try_into()
                .expect("tile_grid_size as u16"),
            c_wid,
            edges_blocked,
        })
    }

    fn is_edge_blocked(&self, tileset_x: u8, tileset_y: u8, dir: Direction) -> bool {
        if let Some(edges_blocked) = &self.edges_blocked {
            let dir_value: usize = match dir {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 2,
                Direction::West => 3,
            };
            let tile_id = tileset_y as usize * self.c_wid + tileset_x as usize;
            let dest_bit = dir_value + 4 * (tile_id & 1);
            let dest_byte = tile_id / 2;
            edges_blocked[dest_byte] & (1 << dest_bit) != 0
        } else {
            false
        }
    }
}

impl TilesetLoader {
    fn new(tileset_defs_json: &[ldtk::TilesetDefinition]) -> Self {
        let mut tileset_handles = HashMap::new();
        for tileset_json in tileset_defs_json {
            if tileset_json.rel_path.is_some() {
                tileset_handles.insert(tileset_json.uid, Weak::new());
            }
        }

        Self(RefCell::new(tileset_handles))
    }

    fn get_tileset(
        &self,
        res: &Resources,
        tileset_defs_json: &[ldtk::TilesetDefinition],
        edge_blocked_enum_uid: i64,
        uid: i64,
    ) -> Option<Rc<Tileset>> {
        let mut tileset_handles = self.0.borrow_mut();
        let tileset_handle = tileset_handles
            .get_mut(&uid)
            .expect("uid in tileset_handles");
        if let Some(tileset_handle) = tileset_handle.upgrade() {
            Some(tileset_handle)
        } else {
            let tileset_json = &tileset_defs_json
                .iter()
                .find(|t| t.uid == uid)
                .expect("uid in tileset definitions");
            let tileset = Rc::new(Tileset::new(res, edge_blocked_enum_uid, tileset_json)?);
            *tileset_handle = Rc::downgrade(&tileset);
            Some(tileset)
        }
    }
}
