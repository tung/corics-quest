use crate::actor::*;
use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::direction::*;
use crate::input::*;
use crate::levels::TILE_SIZE;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct WalkAround;

pub enum WalkAroundEvent {
    DebugQuit,
    TalkActor(usize),
}

impl WalkAround {
    pub fn new() -> Self {
        Self
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
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> WalkAroundEvent {
        loop {
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
                if mctx.level.is_edge_blocked(grid_x, grid_y, dir)
                    || npc_actor_at(mctx, grid_x, grid_y, dir).is_some()
                {
                    mctx.actors[0].stop_walk_animation();
                } else {
                    walk_player(&mut mctx.actors[..], dir).await;
                }
            } else {
                mctx.actors[0].stop_walk_animation();

                if mctx.input.is_key_pressed(GameKey::DebugQuit) {
                    return WalkAroundEvent::DebugQuit;
                } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                    let Actor {
                        grid_x,
                        grid_y,
                        face_dir,
                        ..
                    } = mctx.actors[0];

                    if let Some(npc) = npc_actor_at(mctx, grid_x, grid_y, face_dir) {
                        return WalkAroundEvent::TalkActor(npc);
                    }
                }
            }
        }
    }
}

fn npc_actor_at(
    mctx: &ModeContext<'_, '_>,
    grid_x: i32,
    grid_y: i32,
    face_dir: Direction,
) -> Option<usize> {
    mctx.actors
        .iter()
        .skip(1)
        .position(|a| a.grid_x == grid_x + face_dir.dx() && a.grid_y == grid_y + face_dir.dy())
        .map(|i| i + 1)
}
