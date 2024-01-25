use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::text::*;

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
        dctx.level.draw(dctx.gctx, **dctx.camera_x, **dctx.camera_y);
        self.debug_text.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> WalkAroundEvent {
        loop {
            self.debug_text.set_text(
                mctx.gctx,
                mctx.res,
                &format!("{}\n{}", **mctx.camera_x, **mctx.camera_y),
            );
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
