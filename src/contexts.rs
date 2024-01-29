use crate::actor::*;
use crate::async_utils::SharedMut;
use crate::enemy::*;
use crate::input::*;
use crate::levels::*;
use crate::modes::*;
use crate::progress::*;
use crate::resources::*;

use miniquad::graphics::GraphicsContext;

macro_rules! update_mode {
    ($name:ident, $event:ident) => {
        pub async fn $name(&mut self) -> $event {
            let gctx = self.gctx();
            self.modes
                .$name(&mut ModeContext {
                    gctx,
                    res: &self.res,
                    input: &mut self.input,
                    progress: &mut self.progress,
                    level: &mut self.level,
                    actors: &mut self.actors,
                })
                .await
        }
    };
}

pub struct DrawContext<'a, 'g> {
    pub gctx: &'g mut GraphicsContext,
    pub level: &'a SharedMut<Level>,
    pub actors: &'a SharedMut<Vec<Actor>>,
}

pub struct ModeContext<'a, 'g> {
    pub gctx: &'g mut GraphicsContext,
    pub res: &'a Resources,
    pub input: &'a mut SharedMut<Input>,
    pub progress: &'a mut SharedMut<Progress>,
    pub level: &'a mut SharedMut<Level>,
    pub actors: &'a mut SharedMut<Vec<Actor>>,
}

/// Code that polls async code should use this by moving a clone of it into the async code.
/// The polling code and async code can then communicate by accessing the data within.
pub struct ScriptContext {
    gctx_ptr: SharedMut<*mut GraphicsContext>,
    pub modes: SharedMut<ModeStack>,
    pub input: SharedMut<Input>,
    pub res: Resources,
    pub progress: SharedMut<Progress>,
    pub level: SharedMut<Level>,
    pub actors: SharedMut<Vec<Actor>>,
}

impl ScriptContext {
    pub fn new(gctx: &mut GraphicsContext, res: Resources) -> Self {
        let (level, mut actors) = res.levels.level_by_identifier(gctx, &res, "Start");
        actors.insert(
            0,
            Actor::new(gctx, &res, ActorType::Player, 6, 3, "coric.png"),
        );

        Self {
            gctx_ptr: SharedMut::new(std::ptr::null_mut()),
            modes: SharedMut::new(ModeStack::new()),
            input: SharedMut::new(Input::new()),
            res,
            progress: SharedMut::new(Progress::new()),
            level: SharedMut::new(level),
            actors: SharedMut::new(actors),
        }
    }

    /// # Safety
    ///
    /// Use this only to send a clone into async code to communicate with it.
    /// Any other use is probably unsound.
    pub unsafe fn clone(this: &Self) -> Self {
        Self {
            gctx_ptr: SharedMut::clone(&this.gctx_ptr),
            modes: SharedMut::clone(&this.modes),
            input: SharedMut::clone(&this.input),
            res: this.res.clone(),
            progress: SharedMut::clone(&this.progress),
            level: SharedMut::clone(&this.level),
            actors: SharedMut::clone(&this.actors),
        }
    }

    pub fn draw_context<'a, 'g>(&'a self, gctx: &'g mut GraphicsContext) -> DrawContext<'a, 'g> {
        DrawContext {
            gctx,
            level: &self.level,
            actors: &self.actors,
        }
    }

    fn gctx(&self) -> &'static mut GraphicsContext {
        // SAFETY: This function is only ever called in the async script during [App::update],
        // which sets `gctx` to its [miniquad::graphics::GraphicsContext] before polling the
        // async script, and unsets it immediately afterwards.
        //
        // The `'static` lifetime of the return type is a big fat lie, but is needed for good
        // ergononmics; it's not safe to hold across await points in the async script, but
        // we'll avoid that problem by just never doing that.
        unsafe { self.gctx_ptr.as_mut().unwrap() }
    }

    pub fn set_gctx(&mut self, gctx: &mut GraphicsContext) {
        *self.gctx_ptr = gctx as *mut GraphicsContext;
    }

    pub fn unset_gctx(&mut self) {
        *self.gctx_ptr = std::ptr::null_mut();
    }

    pub fn pop_mode(&mut self) {
        self.modes.pop();
    }

    pub fn push_battle_mode(&mut self, enemy: Enemy) {
        let gctx = self.gctx();
        self.modes.push(Battle::new(gctx, &self.res, enemy));
    }

    pub fn push_main_menu_mode(&mut self) {
        let gctx = self.gctx();
        self.modes
            .push(MainMenu::new(gctx, &self.res, &self.progress));
    }

    pub fn push_text_box_mode(&mut self, s: &str) {
        let gctx = self.gctx();
        self.modes.push(TextBox::new(gctx, &self.res, s));
    }

    pub fn push_walk_around_mode(&mut self) {
        self.modes.push(WalkAround::new());
    }

    update_mode!(update_battle_mode, BattleEvent);
    update_mode!(update_main_menu_mode, MainMenuEvent);
    update_mode!(update_text_box_mode, TextBoxEvent);
    update_mode!(update_walk_around_mode, WalkAroundEvent);
}
