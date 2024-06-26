mod battle;
mod debug_menu;
mod ending;
mod intro;
mod main_menu;
mod text_box;
mod title;
mod walk_around;
mod yes_no_prompt;

pub use battle::*;
pub use debug_menu::*;
pub use ending::*;
pub use intro::*;
pub use main_menu::*;
pub use text_box::*;
pub use title::*;
pub use walk_around::*;
pub use yes_no_prompt::*;

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
                    Some(_) => {
                        panic!(
                            "{} called on a mode that isn't {}",
                            stringify!($update),
                            stringify!($sub_mode),
                        );
                    }
                    None => panic!("{} called on an empty mode stack", stringify!($update)),
                }
            }
        }
    };
}

pub enum Mode {
    Battle(Box<Battle>),
    DebugMenu(Box<DebugMenu>),
    Ending(Box<Ending>),
    Intro(Box<Intro>),
    MainMenu(Box<MainMenu>),
    TextBox(Box<TextBox>),
    Title(Box<Title>),
    WalkAround(Box<WalkAround>),
    YesNoPrompt(Box<YesNoPrompt>),
}

impl_mode!(Battle, BattleEvent, update_battle_mode);
impl_mode!(DebugMenu, DebugMenuEvent, update_debug_menu_mode);
impl_mode!(Ending, EndingEvent, update_ending_mode);
impl_mode!(Intro, IntroEvent, update_intro_mode);
impl_mode!(MainMenu, MainMenuEvent, update_main_menu_mode);
impl_mode!(TextBox, TextBoxEvent, update_text_box_mode);
impl_mode!(Title, TitleEvent, update_title_mode);
impl_mode!(WalkAround, WalkAroundEvent, update_walk_around_mode);
impl_mode!(YesNoPrompt, YesNoPromptEvent, update_yes_no_prompt_mode);

pub struct ModeStack(Vec<Mode>);

impl Mode {
    pub fn draw(&self, dctx: &mut DrawContext) {
        use Mode::*;

        match self {
            Battle(m) => m.draw(dctx),
            DebugMenu(m) => m.draw(dctx),
            Ending(m) => m.draw(dctx),
            Intro(m) => m.draw(dctx),
            MainMenu(m) => m.draw(dctx),
            TextBox(m) => m.draw(dctx),
            Title(m) => m.draw(dctx),
            WalkAround(m) => m.draw(dctx),
            YesNoPrompt(m) => m.draw(dctx),
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
