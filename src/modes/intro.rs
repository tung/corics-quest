use crate::contexts::*;
use crate::resources::*;
use crate::sprite::*;

use miniquad::GlContext;

pub struct Intro {
    air: Sprite,
}

pub enum IntroEvent {
    Done,
}

impl Intro {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        Self {
            air: Sprite::new(gctx, res, "air.png"),
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.air.draw(dctx.gctx, 144, 56);
    }

    pub async fn update(&mut self, _mctx: &mut ModeContext<'_, '_>) -> IntroEvent {
        IntroEvent::Done
    }
}
