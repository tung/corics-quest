use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::text::*;
use crate::window::*;

use miniquad::GlContext;

pub struct TextBox {
    window: Window,
    text: Text,
}

pub enum TextBoxEvent {
    Done,
}

impl TextBox {
    pub fn new(gctx: &mut GlContext, res: &Resources, s: &str) -> Self {
        Self {
            window: Window::new(gctx, res, 44, 124, 232, 40),
            text: Text::from_str(gctx, res, 52, 132, s),
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.window.draw(dctx.gctx);
        self.text.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> TextBoxEvent {
        self.text.reveal().await;
        loop {
            wait_once().await;
            if mctx.input.is_key_pressed(GameKey::Confirm) {
                mctx.audio.play_sfx(Sfx::Cursor);
                return TextBoxEvent::Done;
            }
        }
    }
}
