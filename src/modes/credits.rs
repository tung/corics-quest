use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::resources::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Credits {
    credits_text: Text,
    version_text: Text,
}

pub enum CreditsEvent {
    Done,
}

const CREDITS_TEXT: &str = r#"Coric's Quest

Created By
  tungtn (tung.github.io)

Artwork
  Lanea Zimmerman (OpenGameArt.org)

Enemy Artwork
  David E. Gervais (pousse.rapiere.free.fr/tome/)

Music
  Avgvsta (OpenGameArt.org)
  Yubatake (OpenGameArt.org)

Sound Effects
  jfxr.frozenfractal.com

Made with Rust and Miniquad
"#;

impl Credits {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        Self {
            credits_text: Text::from_str(gctx, res, 6, 4, CREDITS_TEXT),
            version_text: Text::from_str(
                gctx,
                res,
                6 + 14 * 6,
                4,
                &format!("({})", env!("CARGO_PKG_VERSION")),
            ),
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.credits_text.draw(dctx.gctx);
        self.version_text.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> CreditsEvent {
        mctx.fade.in_from_black(60).await;

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel)
                || mctx.input.is_key_pressed(GameKey::Confirm)
            {
                mctx.audio.play_sfx(Sfx::Cancel);
                mctx.fade.out_to_black(60).await;
                return CreditsEvent::Done;
            }
        }
    }
}
