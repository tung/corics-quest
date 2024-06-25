use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::resources::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Ending {
    texts: [Text; 4],
}

pub enum EndingEvent {
    Done,
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
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        for t in &self.texts {
            t.draw(dctx.gctx);
        }
    }

    pub async fn update(&mut self, _mctx: &mut ModeContext<'_, '_>) -> EndingEvent {
        for t in &mut self.texts {
            t.hide_all_chars();
        }

        for t in &mut self.texts {
            while !t.all_chars_shown() {
                for _ in 0..5 {
                    wait_once().await;
                }
                t.show_one_char();
            }

            for _ in 0..150 {
                wait_once().await;
            }
        }

        EndingEvent::Done
    }
}
