use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::input::*;

pub struct WalkAround;

pub enum WalkAroundEvent {
    DebugQuit,
}

impl WalkAround {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        dctx.level.draw(dctx.gctx, **dctx.camera_x, **dctx.camera_y);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> WalkAroundEvent {
        loop {
            wait_once().await;
            if mctx.input.is_key_down(GameKey::DebugQuit) {
                return WalkAroundEvent::DebugQuit;
            }
            if mctx.input.is_key_down(GameKey::Up) {
                **mctx.camera_y -= 1;
            }
            if mctx.input.is_key_down(GameKey::Down) {
                **mctx.camera_y += 1;
            }
            if mctx.input.is_key_down(GameKey::Left) {
                **mctx.camera_x -= 1;
            }
            if mctx.input.is_key_down(GameKey::Right) {
                **mctx.camera_x += 1;
            }
        }
    }
}
