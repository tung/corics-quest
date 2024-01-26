use crate::actor::*;
use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::direction::*;
use crate::input::*;
use crate::levels::TILE_SIZE;
use crate::resources::*;
use crate::text::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct WalkAround {
    debug_text: Text,
}

pub enum WalkAroundEvent {
    DebugQuit,
}

impl WalkAround {
    pub fn new(res: &Resources) -> Self {
        Self {
            debug_text: Text::new(res, 0, 0),
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        let Actor {
            grid_x,
            grid_y,
            offset_x,
            offset_y,
            ..
        } = dctx.actors[0];

        let camera_x = if dctx.level.px_wid >= SCREEN_WIDTH as i32 {
            let half_width = SCREEN_WIDTH as i32 / 2;
            (TILE_SIZE * grid_x + offset_x + TILE_SIZE / 2)
                .max(half_width)
                .min(dctx.level.px_wid - half_width)
        } else {
            dctx.level.px_wid / 2
        };

        let camera_y = if dctx.level.px_hei >= SCREEN_HEIGHT as i32 {
            let half_height = SCREEN_HEIGHT as i32 / 2;
            (TILE_SIZE * grid_y + offset_y + TILE_SIZE / 2)
                .max(half_height)
                .min(dctx.level.px_hei - half_height)
        } else {
            dctx.level.px_hei / 2
        };

        dctx.level.draw(dctx.gctx, camera_x, camera_y);
        for actor in dctx.actors.iter() {
            actor.draw(dctx.gctx, camera_x, camera_y);
        }
        self.debug_text.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> WalkAroundEvent {
        loop {
            self.debug_text.set_text(
                mctx.gctx,
                mctx.res,
                &format!("{}\n{}", mctx.actors[0].grid_x, mctx.actors[0].grid_y),
            );

            wait_once().await;

            let player_move = if mctx.input.is_key_down(GameKey::Up) {
                Some(Direction::North)
            } else if mctx.input.is_key_down(GameKey::Right) {
                Some(Direction::East)
            } else if mctx.input.is_key_down(GameKey::Down) {
                Some(Direction::South)
            } else if mctx.input.is_key_down(GameKey::Left) {
                Some(Direction::West)
            } else {
                None
            };
            if let Some(dir) = player_move {
                mctx.actors[0].face_dir = dir;
                mctx.actors[0].start_walk_animation(dir);

                let Actor { grid_x, grid_y, .. } = mctx.actors[0];
                if mctx.level.is_edge_blocked(grid_x, grid_y, dir) {
                    mctx.actors[0].stop_walk_animation();
                } else {
                    walk_player(&mut mctx.actors[..], dir).await;
                }
            } else {
                mctx.actors[0].stop_walk_animation();

                if mctx.input.is_key_down(GameKey::DebugQuit) {
                    return WalkAroundEvent::DebugQuit;
                }
            }
        }
    }
}
