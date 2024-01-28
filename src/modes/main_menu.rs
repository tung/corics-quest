use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::input::*;
use crate::progress::*;
use crate::resources::*;
use crate::text::*;
use crate::window::*;

use miniquad::graphics::GraphicsContext;

pub struct MainMenu {
    status_window: Window,
    status_text: Text,
    menu_window: Window,
    menu_text: Text,
    menu_cursor: Text,
    selection: i32,
}

pub enum MainMenuEvent {
    Done,
}

impl MainMenu {
    pub fn new(gctx: &mut GraphicsContext, res: &Resources, progress: &Progress) -> Self {
        let status_window = Window::new(gctx, res, 44, 44, 180, 90);
        let status_text = Text::from_str(
            gctx,
            res,
            52,
            52,
            &format!(
                "Coric\n\
                 \n\
                 Level: {}\n\
                 HP: {} / {}\n\
                 MP: {} / {}\n\
                 Attack: {}\n\
                 Defense: {}\n\
                 Experience: {}",
                progress.level,
                progress.hp,
                progress.max_hp,
                progress.mp,
                progress.max_mp,
                progress.attack,
                progress.defense,
                progress.exp,
            ),
        );
        let menu_window = Window::new(gctx, res, 234, 44, 60, 56);
        let menu_text = Text::from_str(gctx, res, 248, 52, "RETURN\n\nMAGIC\n\nITEM");
        let menu_cursor = Text::from_str(gctx, res, 240, 52, "►");

        Self {
            status_window,
            status_text,
            menu_window,
            menu_text,
            menu_cursor,
            selection: 0,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.status_window.draw(dctx.gctx);
        self.status_text.draw(dctx.gctx);
        self.menu_window.draw(dctx.gctx);
        self.menu_text.draw(dctx.gctx);
        self.menu_cursor.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> MainMenuEvent {
        loop {
            wait_once().await;

            if (mctx.input.is_key_pressed(GameKey::Confirm) && self.selection == 0)
                || mctx.input.is_key_pressed(GameKey::Cancel)
            {
                return MainMenuEvent::Done;
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                if self.selection == 0 {
                    self.selection = 2;
                } else {
                    self.selection -= 1;
                }
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                if self.selection == 2 {
                    self.selection = 0;
                } else {
                    self.selection += 1;
                }
            }
            self.menu_cursor.set_offset(240, 52 + 16 * self.selection);
        }
    }
}
