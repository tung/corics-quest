use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::sprite::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Title {
    title: Sprite,
    menu_text: Text,
    cursor: Text,
}

pub enum TitleEvent {
    NewGame,
}

const TITLE_X: i32 = 120;
const TITLE_Y: i32 = 38;
const MENU_X: i32 = 136;
const MENU_Y: i32 = TITLE_Y + 36 + 38;
const CURSOR_X: i32 = MENU_X - 8;
const CURSOR_Y: i32 = MENU_Y;

impl Title {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        let mut menu_text = Text::new(res, MENU_X, MENU_Y);

        menu_text.set_text(gctx, res, "New Game");

        Self {
            title: Sprite::new(gctx, res, "title.png"),
            menu_text,
            cursor: Text::from_str(gctx, res, CURSOR_X, CURSOR_Y, "►"),
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.title.draw(dctx.gctx, TITLE_X, TITLE_Y);
        self.menu_text.draw(dctx.gctx);
        self.cursor.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> TitleEvent {
        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Confirm) {
                return TitleEvent::NewGame;
            }

            self.title.animate();
        }
    }
}
