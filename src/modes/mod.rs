mod walk_around;

pub use walk_around::*;

use crate::contexts::*;

macro_rules! impl_mode {
    ($sub_mode:ident, $event:ident, $update:ident) => {
        impl From<$sub_mode> for Mode {
            fn from(f: $sub_mode) -> Self {
                Self::$sub_mode(Box::new(f))
            }
        }

        impl ModeStack {
            pub async fn $update(&mut self, mctx: &mut ModeContext<'_, '_>) -> $event {
                match self.0.last_mut() {
                    Some(Mode::$sub_mode(m)) => m.update(mctx).await,
                    //Some(_) => {
                    //    panic!(
                    //        "{} called on a mode that isn't {}",
                    //        stringify!($update),
                    //        stringify!($sub_mode),
                    //    );
                    //}
                    None => panic!("{} called on an empty mode stack", stringify!($update)),
                }
            }
        }
    };
}

pub enum Mode {
    WalkAround(Box<WalkAround>),
}

impl_mode!(WalkAround, WalkAroundEvent, update_walk_around_mode);

pub struct ModeStack(Vec<Mode>);

impl Mode {
    pub fn draw(&self, dctx: &mut DrawContext) {
        use Mode::*;

        match self {
            WalkAround(m) => m.draw(dctx),
        }
    }
}

impl ModeStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        for mode in &self.0 {
            mode.draw(dctx);
        }
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn push<M: Into<Mode>>(&mut self, mode: M) {
        self.0.push(mode.into());
    }
}
