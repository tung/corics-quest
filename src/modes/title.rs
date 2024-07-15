use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::progress::*;
use crate::resources::*;
use crate::sprite::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Title {
    title: Sprite,
    confirm_text: Text,
    menu_text: Text,
    cursor: Text,
    can_continue: bool,
    confirm_pressed: bool,
    selection: i32,
}

pub enum TitleEvent {
    NewGame(bool),
    Continue,
    Options,
}

const TITLE_X: i32 = 120;
const TITLE_Y: i32 = 38;
const CONFIRM_X: i32 = 127;
const CONFIRM_Y: i32 = TITLE_Y + 36 + 46;
const MENU_X: i32 = 136;
const MENU_Y: i32 = TITLE_Y + 36 + 38;
const CURSOR_X: i32 = MENU_X - 8;
const CURSOR_Y: i32 = MENU_Y;

impl Title {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        let can_continue = save_data_exists();

        Self {
            title: Sprite::new(gctx, res, "title.png"),
            confirm_text: Text::new(res, CONFIRM_X, CONFIRM_Y),
            menu_text: Text::new(res, MENU_X, MENU_Y),
            cursor: Text::new(res, CURSOR_X, CURSOR_Y),
            can_continue,
            confirm_pressed: false,
            selection: can_continue as i32,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.title.draw(dctx.gctx, TITLE_X, TITLE_Y);
        self.confirm_text.draw(dctx.gctx);
        self.menu_text.draw(dctx.gctx);
        self.cursor.draw(dctx.gctx);
    }

    fn num_menu_entries(&self) -> i32 {
        2 + self.can_continue as i32
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> TitleEvent {
        if !self.confirm_pressed {
            mctx.fade.in_from_black(60).await;
            self.confirm_text
                .set_text(mctx.gctx, mctx.res, "Press Space");
            self.confirm_text.reveal().await;
            loop {
                wait_once().await;
                if mctx.input.is_key_pressed(GameKey::Confirm) {
                    self.confirm_pressed = true;
                    self.confirm_text.set_text(mctx.gctx, mctx.res, "");
                    mctx.audio.play_sfx(Sfx::Confirm);
                    break;
                }
            }
        }

        self.menu_text.set_text(
            mctx.gctx,
            mctx.res,
            if self.can_continue {
                "New Game\nContinue\nOptions"
            } else {
                "New Game\nOptions"
            },
        );

        self.cursor.set_text(mctx.gctx, mctx.res, "►");
        self.update_cursor_pos();

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Confirm) {
                mctx.audio.play_sfx(Sfx::Confirm);
                let real_selection = if !self.can_continue && self.selection >= 1 {
                    self.selection + 1
                } else {
                    self.selection
                };
                return match real_selection {
                    0 => TitleEvent::NewGame(true),
                    1 => TitleEvent::Continue,
                    2 => {
                        self.menu_text.set_text(mctx.gctx, mctx.res, "");
                        self.cursor.set_text(mctx.gctx, mctx.res, "");
                        TitleEvent::Options
                    }
                    _ => unreachable!(),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == 0 {
                    self.selection = self.num_menu_entries() - 1;
                } else {
                    self.selection -= 1;
                }
                self.update_cursor_pos();
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == self.num_menu_entries() - 1 {
                    self.selection = 0;
                } else {
                    self.selection += 1;
                }
                self.update_cursor_pos();
            } else if mctx.input.is_key_pressed(GameKey::DebugMenu) && self.selection == 0 {
                mctx.audio.play_sfx(Sfx::Confirm);
                return TitleEvent::NewGame(false);
            }

            self.title.animate();
        }
    }

    fn update_cursor_pos(&mut self) {
        self.cursor
            .set_offset(CURSOR_X, CURSOR_Y + 8 * self.selection);
    }
}
