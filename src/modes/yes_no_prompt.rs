use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::text::*;
use crate::window::*;

use miniquad::GlContext;

pub struct YesNoPrompt {
    window: Window,
    text: Text,
    cursor: Text,
    choice: bool,
    no_x_diff: i32,
}

pub enum YesNoPromptEvent {
    Yes,
    No,
}

const CURSOR_X: i32 = 52;
const CURSOR_Y: i32 = 148;

impl YesNoPrompt {
    pub fn new(
        gctx: &mut GlContext,
        res: &Resources,
        prompt: &str,
        yes_label: &str,
        no_label: &str,
        initial_choice: bool,
    ) -> Self {
        let no_x_diff = 5 + yes_label.len() as i32;

        Self {
            window: Window::new(gctx, res, 44, 124, 232, 40),
            text: Text::from_str(
                gctx,
                res,
                52,
                132,
                &format!("{prompt}\n\n {yes_label}     {no_label}"),
            ),
            cursor: Text::from_str(gctx, res, 0, 0, "►"),
            choice: initial_choice,
            no_x_diff,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.window.draw(dctx.gctx);
        self.text.draw(dctx.gctx);
        self.cursor.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> YesNoPromptEvent {
        self.update_cursor_pos();
        self.cursor.hide_all_chars();
        self.text.reveal().await;
        self.cursor.reveal().await;

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Confirm) {
                return if self.choice {
                    mctx.audio.play_sfx(Sfx::Confirm);
                    YesNoPromptEvent::Yes
                } else {
                    mctx.audio.play_sfx(Sfx::Cancel);
                    YesNoPromptEvent::No
                };
            } else if mctx.input.is_key_pressed(GameKey::Left) {
                mctx.audio.play_sfx(Sfx::Cursor);
                self.choice = true;
                self.update_cursor_pos();
            } else if mctx.input.is_key_pressed(GameKey::Right) {
                mctx.audio.play_sfx(Sfx::Cursor);
                self.choice = false;
                self.update_cursor_pos();
            }
        }
    }

    fn update_cursor_pos(&mut self) {
        self.cursor.set_offset(
            CURSOR_X + 6 * self.no_x_diff * (1 - self.choice as i32),
            CURSOR_Y,
        );
    }
}
