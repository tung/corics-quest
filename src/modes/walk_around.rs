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
    DebugLevelUp,
    DebugSteps(i32),
    DebugEquip(i32),
    DebugBattle(i32),
    DebugMenu,
    Encounter,
    MainMenu,
    TalkActor(usize),
    TouchLevelEdge(Direction),
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
                let c_level_wid = mctx.level.px_wid / TILE_SIZE;
                let c_level_hei = mctx.level.px_hei / TILE_SIZE;

                if mctx.level.is_edge_blocked(grid_x, grid_y, dir)
                    || npc_actor_at(mctx, grid_x, grid_y, dir).is_some()
                {
                    mctx.actors[0].stop_walk_animation();
                } else {
                    let in_bounds =
                        grid_x >= 0 && grid_x < c_level_wid && grid_y >= 0 && grid_y < c_level_hei;
                    if in_bounds
                        && (grid_y == 0 && dir == Direction::North
                            || grid_x == c_level_wid - 1 && dir == Direction::East
                            || grid_y == c_level_hei - 1 && dir == Direction::South
                            || grid_x == 0 && dir == Direction::West)
                    {
                        return WalkAroundEvent::TouchLevelEdge(dir);
                    } else {
                        walk_player(&mut mctx.actors[..], dir, None).await;

                        *mctx.steps += 1;

                        // slide over ice tiles until level or blocking tile edge is reached
                        loop {
                            let Actor { grid_x, grid_y, .. } = mctx.actors[0];
                            let next_x = grid_x + dir.dx();
                            let next_y = grid_y + dir.dy();
                            let next_step_in_bounds = next_x >= 0
                                && next_x < c_level_wid
                                && next_y >= 0
                                && next_y < c_level_hei;

                            if !next_step_in_bounds
                                || !mctx.level.is_ice_tile(grid_x, grid_y)
                                || mctx.level.is_edge_blocked(grid_x, grid_y, dir)
                                || npc_actor_at(mctx, grid_x, grid_y, dir).is_some()
                            {
                                break;
                            }

                            mctx.actors[0].stop_walk_animation();
                            walk_player(&mut mctx.actors[..], dir, None).await;
                        }
                    }
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
                } else if mctx.input.is_key_pressed(GameKey::Cancel) {
                    return WalkAroundEvent::MainMenu;
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle) {
                    return WalkAroundEvent::Encounter;
                } else if mctx.input.is_key_pressed(GameKey::DebugLevelUp) {
                    return WalkAroundEvent::DebugLevelUp;
                } else if mctx.input.is_key_pressed(GameKey::DebugSteps) {
                    let steps = *mctx.steps;
                    *mctx.steps = 0;
                    return WalkAroundEvent::DebugSteps(steps);
                } else if mctx.input.is_key_pressed(GameKey::DebugEquip1) {
                    return WalkAroundEvent::DebugEquip(1);
                } else if mctx.input.is_key_pressed(GameKey::DebugEquip2) {
                    return WalkAroundEvent::DebugEquip(2);
                } else if mctx.input.is_key_pressed(GameKey::DebugEquip3) {
                    return WalkAroundEvent::DebugEquip(3);
                } else if mctx.input.is_key_pressed(GameKey::DebugEquip4) {
                    return WalkAroundEvent::DebugEquip(4);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle1) {
                    return WalkAroundEvent::DebugBattle(1);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle2) {
                    return WalkAroundEvent::DebugBattle(2);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle3) {
                    return WalkAroundEvent::DebugBattle(3);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle4) {
                    return WalkAroundEvent::DebugBattle(4);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle5) {
                    return WalkAroundEvent::DebugBattle(5);
                } else if mctx.input.is_key_pressed(GameKey::DebugBattle6) {
                    return WalkAroundEvent::DebugBattle(6);
                } else if mctx.input.is_key_pressed(GameKey::DebugMenu) {
                    return WalkAroundEvent::DebugMenu;
                }
            }
        }
    }
}

fn npc_actor_at(
    mctx: &ModeContext,
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
