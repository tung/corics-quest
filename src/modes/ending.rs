use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Ending {
    texts: [Text; 4],
    text_shown: bool,
}

pub enum EndingEvent {
    Credits,
}

impl Ending {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        Self {
            texts: [
                Text::from_str(
                    gctx,
                    res,
                    64,
                    32,
                    "With the Spirits wrested from\n\
                     the clutches of darkness, peace\n\
                     returned once more to the land.",
                ),
                Text::from_str(
                    gctx,
                    res,
                    64,
                    72,
                    "But what of the malevolent force\n\
                     that had come to possess them?",
                ),
                Text::from_str(
                    gctx,
                    res,
                    64,
                    104,
                    "Whenever evil rises, so, too,\n\
                     shall a hero rise to defeat it…",
                ),
                Text::from_str(gctx, res, 106, 136, "~ T H E   E N D ~"),
            ],
            text_shown: false,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        for t in &self.texts {
            t.draw(dctx.gctx);
        }
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> EndingEvent {
        if self.text_shown {
            for t in &mut self.texts {
                t.show_all_chars();
            }
            mctx.fade.in_from_black(60).await;
            mctx.audio.set_music_volume_scripted(100);
        } else {
            mctx.audio.play_music(Some(Music::Ending)).await;

            for t in &mut self.texts {
                t.hide_all_chars();
            }

            while !self.texts[0].all_chars_shown() {
                for _ in 0..5 {
                    wait_once().await;
                }
                self.texts[0].show_one_char();
            }
            for t in &mut self.texts[1..] {
                for _ in 0..150 {
                    wait_once().await;
                }

                while !t.all_chars_shown() {
                    for _ in 0..5 {
                        wait_once().await;
                    }
                    t.show_one_char();
                }
            }

            self.text_shown = true;
        }

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Confirm) {
                mctx.audio.play_sfx(Sfx::Confirm);
                mctx.audio.set_music_volume_scripted(40);
                mctx.fade.out_to_black(60).await;
                for t in &mut self.texts {
                    t.hide_all_chars();
                }
                return EndingEvent::Credits;
            }
        }
    }
}
