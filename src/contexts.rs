use crate::actor::*;
use crate::async_utils::*;
use crate::direction::*;
use crate::enemy::*;
use crate::get_gctx;
use crate::input::*;
use crate::levels::*;
use crate::modes::*;
use crate::progress::*;
use crate::random::*;
use crate::resources::*;

use miniquad::GlContext;

macro_rules! update_mode {
    ($name:ident, $event:ident) => {
        pub async fn $name(&mut self) -> $event {
            let gctx = get_gctx();

            self.modes
                .$name(&mut ModeContext {
                    gctx,
                    res: &self.res,
                    input: &self.input,
                    rng: &mut self.rng,
                    progress: &mut self.progress,
                    level: &self.level,
                    actors: &mut self.actors,
                    fade: &self.fade,
                    steps: &mut self.steps,
                })
                .await
        }
    };
}

pub struct DrawContext<'a, 'g> {
    pub gctx: &'g mut GlContext,
    pub level: &'a SharedMut<Level>,
    pub actors: &'a SharedMut<Vec<Actor>>,
}

pub struct ModeContext<'a, 'g> {
    pub gctx: &'g mut GlContext,
    pub res: &'a Resources,
    pub input: &'a SharedMut<Input>,
    pub rng: &'a mut Rng,
    pub progress: &'a mut Progress,
    pub level: &'a SharedMut<Level>,
    pub actors: &'a mut SharedMut<Vec<Actor>>,
    pub fade: &'a SharedMut<[f32; 4]>,
    pub steps: &'a mut i32,
}

pub struct ScriptContext {
    pub res: Resources,
    pub input: SharedMut<Input>,
    pub modes: SharedMut<ModeStack>,
    pub rng: Rng,
    pub progress: Progress,
    pub level: SharedMut<Level>,
    pub actors: SharedMut<Vec<Actor>>,
    pub fade: SharedMut<[f32; 4]>,
    pub steps: i32,
}

impl ScriptContext {
    pub fn new(
        res: Resources,
        input: &SharedMut<Input>,
        modes: &SharedMut<ModeStack>,
        level: &SharedMut<Level>,
        actors: &SharedMut<Vec<Actor>>,
        fade: &SharedMut<[f32; 4]>,
    ) -> Self {
        // SAFETY: This is immediately sent into the async script function.
        // Access from outside that async function never goes through this.
        unsafe {
            Self {
                res,
                input: SharedMut::clone(input),
                modes: SharedMut::clone(modes),
                rng: Rng::new(miniquad::date::now() as _),
                progress: Progress::new(),
                level: SharedMut::clone(level),
                actors: SharedMut::clone(actors),
                fade: SharedMut::clone(fade),
                steps: 0,
            }
        }
    }

    pub async fn fade_in(&mut self, frames: u16) {
        let step = self.fade[3] / if frames > 0 { frames as f32 } else { 1.0 };
        while self.fade[3] > 0.0 {
            self.fade[3] = (self.fade[3] - step).max(0.0);
            wait_once().await;
        }
    }

    pub async fn fade_out(&mut self, frames: u16) {
        let step = (1.0 - self.fade[3]) / if frames > 0 { frames as f32 } else { 1.0 };
        while self.fade[3] < 1.0 {
            self.fade[3] = (self.fade[3] + step).min(1.0);
            wait_once().await;
        }
    }

    fn prepare_level_and_actors(&self, level: &mut Level, actors: &mut [Actor]) {
        let gctx = get_gctx();

        // display chest according to open/closed state in progress
        if let Some(chest) = actors.iter_mut().find(|a| a.identifier == ActorType::Chest) {
            let chest_opened = self
                .progress
                .collected_chests
                .iter()
                .map(String::as_str)
                .any(|s| s == level.identifier);
            chest.start_animation(if chest_opened { "open" } else { "closed" });
        }

        // show lever turned to its last position and update the map tiles it controls
        let lever_turned = self
            .progress
            .turned_levers
            .iter()
            .any(|l| l == level.identifier.as_str());
        sync_level_and_actors_with_lever(gctx, lever_turned, level, actors);
    }

    pub fn level_by_identifier(&self, identifier: &str) -> (Level, Vec<Actor>) {
        let gctx = get_gctx();

        let (mut level, mut actors) = self
            .res
            .levels
            .level_by_identifier(gctx, &self.res, identifier);
        self.prepare_level_and_actors(&mut level, &mut actors[..]);
        (level, actors)
    }

    pub fn level_by_neighbour(&self, dir: Direction) -> Option<(Level, Vec<Actor>)> {
        let gctx = get_gctx();

        let Actor { grid_x, grid_y, .. } = self.actors[0];
        let Level {
            px_world_x,
            px_world_y,
            ..
        } = *self.level;

        self.res
            .levels
            .level_by_neighbour(
                gctx,
                &self.res,
                &self.level.neighbours[..],
                px_world_x + grid_x * TILE_SIZE,
                px_world_y + grid_y * TILE_SIZE,
                dir,
            )
            .map(|(mut level, mut actors)| {
                self.prepare_level_and_actors(&mut level, &mut actors[..]);
                (level, actors)
            })
    }

    pub fn lever_is_turned(&self) -> bool {
        self.progress
            .turned_levers
            .iter()
            .any(|l| l == self.level.identifier.as_str())
    }

    pub fn toggle_lever(&mut self) {
        let gctx = get_gctx();

        if self.lever_is_turned() {
            let turned_lever_pos = self
                .progress
                .turned_levers
                .iter()
                .position(|l| l == self.level.identifier.as_str())
                .expect("turned lever position");
            self.progress.turned_levers.swap_remove(turned_lever_pos);
        } else {
            self.progress
                .turned_levers
                .push(self.level.identifier.clone());
        }

        sync_level_and_actors_with_lever(
            gctx,
            self.lever_is_turned(),
            &mut self.level,
            &mut self.actors[..],
        );
    }

    pub fn place_gates(&mut self, tile_x: i32, tile_y: i32) {
        let gctx = get_gctx();

        self.level.place_gates(gctx, tile_x, tile_y);
    }

    pub fn pop_mode(&mut self) {
        self.modes.pop();
    }

    pub fn push_battle_mode(&mut self, enemy: Enemy, boss_fight: bool) {
        let gctx = get_gctx();

        self.modes.push(Battle::new(
            gctx,
            &self.res,
            self.progress.max_hp,
            self.progress.max_mp,
            enemy,
            boss_fight,
        ));
    }

    pub fn push_debug_menu_mode(&mut self) {
        let gctx = get_gctx();

        self.modes.push(DebugMenu::new(gctx, &self.res));
    }

    pub fn push_main_menu_mode(&mut self) {
        let gctx = get_gctx();

        self.modes
            .push(MainMenu::new(gctx, &self.res, &self.progress));
    }

    pub fn push_text_box_mode(&mut self, s: &str) {
        let gctx = get_gctx();

        self.modes.push(TextBox::new(gctx, &self.res, s));
    }

    pub fn push_walk_around_mode(&mut self) {
        self.modes.push(WalkAround::new());
    }

    update_mode!(update_battle_mode, BattleEvent);
    update_mode!(update_debug_menu_mode, DebugMenuEvent);
    update_mode!(update_main_menu_mode, MainMenuEvent);
    update_mode!(update_text_box_mode, TextBoxEvent);
    update_mode!(update_walk_around_mode, WalkAroundEvent);
}

fn sync_level_and_actors_with_lever(
    gctx: &mut GlContext,
    lever_turned: bool,
    level: &mut Level,
    actors: &mut [Actor],
) {
    if let Some(lever) = actors.iter_mut().find(|a| a.identifier == ActorType::Lever) {
        lever.start_animation(if lever_turned { "right" } else { "left" });
    }

    level.sync_props_with_lever(gctx, lever_turned);
}
