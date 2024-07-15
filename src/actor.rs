use crate::async_utils::wait_once;
use crate::direction::*;
use crate::ldtk;
use crate::levels::TILE_SIZE;
use crate::resources::*;
use crate::sprite::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::GlContext;
use miniserde::json;

pub struct Actor {
    pub identifier: ActorType,
    pub grid_x: i32,
    pub grid_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub face_dir: Direction,
    pub visible: bool,
    sprite: Sprite,
    pub chest_type: Option<ChestType>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorType {
    Player,
    Bed,
    Chest,
    Lever,
    Ducille,
    Jace,
    Julis,
    Matero,
    Earth,
    Water,
    Fire,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ChestType {
    FireEdge,
    EarthEdge,
    WaterEdge,
    LongSword,
    ChainVest,
    DuelistSword,
    SteelArmor,
    ValorBlade,
    MythicPlate,
}

impl Actor {
    pub fn new(
        gctx: &mut GlContext,
        res: &Resources,
        identifier: ActorType,
        grid_x: i32,
        grid_y: i32,
        path: &str,
    ) -> Self {
        Self {
            identifier,
            grid_x,
            grid_y,
            offset_x: 0,
            offset_y: 0,
            face_dir: Direction::South,
            visible: true,
            sprite: Sprite::new(gctx, res, path),
            chest_type: None,
        }
    }

    pub fn new_by_json(
        gctx: &mut GlContext,
        res: &Resources,
        tileset_defs_json: &[ldtk::TilesetDefinition],
        entity_json: &ldtk::EntityInstance,
    ) -> Self {
        let tileset_uid = entity_json
            .tile
            .as_ref()
            .expect("entity tileset rectangle")
            .tileset_uid;
        let rel_path = &tileset_defs_json
            .iter()
            .find(|t| t.uid == tileset_uid)
            .expect("entity tileset by uid")
            .rel_path
            .as_ref()
            .expect("entity tileset rel_path");

        let mut actor = Self::new(
            gctx,
            res,
            entity_json.identifier.as_str().into(),
            entity_json.grid[0] as i32,
            entity_json.grid[1] as i32,
            rel_path,
        );

        if actor.identifier == ActorType::Chest {
            let chest_type_field = entity_json
                .field_instances
                .iter()
                .find(|fi| fi.identifier == "ChestType")
                .expect("ChestType field instance for ActorType::Chest");

            actor.chest_type = match &chest_type_field.value {
                Some(json::Value::String(s)) => Some(s.as_str().into()),
                _ => panic!("ChestType value must be a string"),
            };
        }

        actor
    }

    pub fn animate(&mut self) {
        self.sprite.animate();
    }

    pub fn draw(&self, gctx: &mut GlContext, camera_x: i32, camera_y: i32) {
        if self.visible {
            self.sprite.draw(
                gctx,
                (TILE_SIZE * self.grid_x) + self.offset_x + SCREEN_WIDTH as i32 / 2 - camera_x,
                (TILE_SIZE * self.grid_y) + self.offset_y + SCREEN_HEIGHT as i32 / 2 - camera_y,
            );
        }
    }

    pub fn start_animation(&mut self, tag: &str) {
        self.sprite.start_animation(tag);
    }

    pub fn start_walk_animation(&mut self, dir: Direction) {
        self.sprite.start_walk_animation(dir);
    }

    pub fn stop_walk_animation(&mut self) {
        self.sprite.stop_walk_animation();
    }
}

impl From<&str> for ActorType {
    fn from(s: &str) -> Self {
        match s {
            "Player" => Self::Player,
            "Bed" => Self::Bed,
            "Chest" => Self::Chest,
            "Lever" => Self::Lever,
            "Ducille" => Self::Ducille,
            "Jace" => Self::Jace,
            "Julis" => Self::Julis,
            "Matero" => Self::Matero,
            "Earth" => Self::Earth,
            "Water" => Self::Water,
            "Fire" => Self::Fire,
            _ => panic!("unknown actor type: {s}"),
        }
    }
}

impl From<&str> for ChestType {
    fn from(s: &str) -> Self {
        match s {
            "FireEdge" => Self::FireEdge,
            "EarthEdge" => Self::EarthEdge,
            "WaterEdge" => Self::WaterEdge,
            "LongSword" => Self::LongSword,
            "ChainVest" => Self::ChainVest,
            "DuelistSword" => Self::DuelistSword,
            "SteelArmor" => Self::SteelArmor,
            "ValorBlade" => Self::ValorBlade,
            "MythicPlate" => Self::MythicPlate,
            _ => panic!("unknown chest type: {s}"),
        }
    }
}

pub fn animate_actors(actors: &mut [Actor]) {
    for actor in actors {
        actor.animate();
    }
}

pub fn move_actors(actors: &mut [Actor]) {
    for actor in actors {
        actor.offset_x -= actor.offset_x.signum();
        actor.offset_y -= actor.offset_y.signum();
    }
}

pub async fn walk_player<F>(actors: &mut [Actor], dir: Direction, mut func: F)
where
    F: FnMut(u16, u16),
{
    actors[0].grid_x += dir.dx();
    actors[0].grid_y += dir.dy();
    actors[0].offset_x -= dir.dx() * TILE_SIZE;
    actors[0].offset_y -= dir.dy() * TILE_SIZE;

    while actors[0].offset_x != 0 || actors[0].offset_y != 0 {
        move_actors(actors);
        animate_actors(actors);

        let offset_remaining = TILE_SIZE - actors[0].offset_x.abs().max(actors[0].offset_y.abs());
        func(offset_remaining as u16, TILE_SIZE as u16);

        wait_once().await;
    }
}
