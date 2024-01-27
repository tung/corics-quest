use crate::async_utils::wait_once;
use crate::direction::*;
use crate::ldtk;
use crate::levels::TILE_SIZE;
use crate::resources::*;
use crate::sprite::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use miniquad::graphics::GraphicsContext;

pub struct Actor {
    pub grid_x: i32,
    pub grid_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub face_dir: Direction,
    pub visible: bool,
    sprite: Sprite,
}

impl Actor {
    pub fn new(
        gctx: &mut GraphicsContext,
        res: &Resources,
        grid_x: i32,
        grid_y: i32,
        path: &str,
    ) -> Self {
        let mut sprite = Sprite::new(gctx, res, path);
        sprite.start_animation("face_s");

        Self {
            grid_x,
            grid_y,
            offset_x: 0,
            offset_y: 0,
            face_dir: Direction::South,
            visible: true,
            sprite,
        }
    }

    pub fn new_by_json(
        gctx: &mut GraphicsContext,
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

        Self::new(
            gctx,
            res,
            entity_json.grid[0] as i32,
            entity_json.grid[1] as i32,
            rel_path,
        )
    }

    pub fn animate(&mut self) {
        self.sprite.animate();
    }

    pub fn draw(&self, gctx: &mut GraphicsContext, camera_x: i32, camera_y: i32) {
        if self.visible {
            self.sprite.draw(
                gctx,
                (TILE_SIZE * self.grid_x) + self.offset_x + SCREEN_WIDTH as i32 / 2 - camera_x,
                (TILE_SIZE * self.grid_y) + self.offset_y + SCREEN_HEIGHT as i32 / 2 - camera_y,
            );
        }
    }

    pub fn start_walk_animation(&mut self, dir: Direction) {
        self.sprite.start_walk_animation(dir);
    }

    pub fn stop_walk_animation(&mut self) {
        self.sprite.stop_walk_animation();
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

pub async fn walk_player(actors: &mut [Actor], dir: Direction) {
    actors[0].grid_x += dir.dx();
    actors[0].grid_y += dir.dy();
    actors[0].offset_x -= dir.dx() * TILE_SIZE;
    actors[0].offset_y -= dir.dy() * TILE_SIZE;
    while actors[0].offset_x != 0 || actors[0].offset_y != 0 {
        move_actors(actors);
        animate_actors(actors);
        wait_once().await;
    }
}
