use crate::async_utils::SharedMut;
use crate::input::*;
use crate::levels::*;
use crate::modes::*;
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
                    level: &mut self.level,
                    camera_x: &mut self.camera_x,
                    camera_y: &mut self.camera_y,
                })
                .await
        }
    };
}

pub struct DrawContext<'a, 'g> {
    pub gctx: &'g mut GraphicsContext,
    pub level: &'a SharedMut<Level>,
    pub camera_x: &'a SharedMut<i32>,
    pub camera_y: &'a SharedMut<i32>,
}

pub struct ModeContext<'a, 'g> {
    pub gctx: &'g mut GraphicsContext,
    pub res: &'a Resources,
    pub input: &'a mut SharedMut<Input>,
    pub level: &'a mut SharedMut<Level>,
    pub camera_x: &'a mut SharedMut<i32>,
    pub camera_y: &'a mut SharedMut<i32>,
}

/// Code that polls async code should use this by moving a clone of it into the async code.
/// The polling code and async code can then communicate by accessing the data within.
pub struct ScriptContext {
    gctx_ptr: SharedMut<*mut GraphicsContext>,
    pub modes: SharedMut<ModeStack>,
    pub input: SharedMut<Input>,
    pub res: Resources,
    pub level: SharedMut<Level>,
    pub camera_x: SharedMut<i32>,
    pub camera_y: SharedMut<i32>,
}

impl ScriptContext {
    pub fn new(gctx: &mut GraphicsContext, res: Resources) -> Self {
        let level = res.levels.level_by_identifier(gctx, &res, "Start");

        Self {
            gctx_ptr: SharedMut::new(std::ptr::null_mut()),
            modes: SharedMut::new(ModeStack::new()),
            input: SharedMut::new(Input::new()),
            res,
            level: SharedMut::new(level),
            camera_x: SharedMut::new(160),
            camera_y: SharedMut::new(88),
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
            level: SharedMut::clone(&this.level),
            camera_x: SharedMut::clone(&this.camera_x),
            camera_y: SharedMut::clone(&this.camera_y),
        }
    }

    pub fn draw_context<'a, 'g>(&'a self, gctx: &'g mut GraphicsContext) -> DrawContext<'a, 'g> {
        DrawContext {
            gctx,
            level: &self.level,
            camera_x: &self.camera_x,
            camera_y: &self.camera_y,
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

    pub fn push_walk_around_mode(&mut self) {
        self.modes.push(WalkAround::new(&self.res));
    }

    update_mode!(update_walk_around_mode, WalkAroundEvent);
}
