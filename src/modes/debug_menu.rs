use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::enemy::*;
use crate::input::*;
use crate::progress::*;
use crate::resources::*;
use crate::text::*;
use crate::window::*;

use miniquad::GlContext;

pub struct DebugMenu {
    window: Window,
    text: Text,
    cursor: Text,
}

pub enum DebugMenuEvent {
    Cancel,
    GainLevel,
    Battle(i32),
    SetWeapon(Option<Weapon>),
    SetArmor(Option<Armor>),
    GetItems,
    LearnAllMagic,
    ResetStepCounts,
    ChangeFlag(i32),
    Warp {
        level_id: &'static str,
        x: i32,
        y: i32,
    },
    Quit,
}

const CURSOR_X: i32 = 91;
const CURSOR_Y: i32 = 44;

impl DebugMenu {
    pub fn new(gctx: &mut GlContext, res: &Resources) -> Self {
        Self {
            window: Window::new(gctx, res, 83, 36, 154, 104),
            text: Text::new(res, 97, 44),
            cursor: Text::from_str(gctx, res, CURSOR_X, CURSOR_Y, "►"),
        }
    }

    async fn battle_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> Option<i32> {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            "Back\n\
             Wilderness 1\n\
             Wilderness 2\n\
             Wilderness 3\n\
             Earth Castle\n\
             Water Castle\n\
             Fire Castle\n\
             Earth Spirit\n\
             Water Spirit\n\
             Fire Spirit",
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return None;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        None
                    }
                    s => Some(s - 1),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 9;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 9 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.window.draw(dctx.gctx);
        self.text.draw(dctx.gctx);
        self.cursor.draw(dctx.gctx);
    }

    async fn flags_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> Option<i32> {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            &format!(
                "Back\n\
                 earth_defeated: {}\n\
                 water_defeated: {}\n\
                 fire_defeated:  {}",
                mctx.progress.earth_defeated,
                mctx.progress.water_defeated,
                mctx.progress.fire_defeated,
            ),
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return None;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        None
                    }
                    s => Some(s - 1),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 3;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 3 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }

    async fn set_armor_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> Option<Option<Armor>> {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            "Back\n\
             (none)\n\
             Leather Armor\n\
             Chain Vest\n\
             Steel Armor\n\
             Mythic Plate",
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return None;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        None
                    }
                    1 => Some(None),
                    s => Some(Some(match s {
                        2 => Armor {
                            name: String::from("Leather Armor"),
                            defense: 2,
                        },
                        3 => Armor {
                            name: String::from("Chain Vest"),
                            defense: 7,
                        },
                        4 => Armor {
                            name: String::from("Steel Armor"),
                            defense: 13,
                        },
                        5 => Armor {
                            name: String::from("Mythic Plate"),
                            defense: 25,
                        },
                        _ => panic!("invalid armor selection"),
                    })),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 5;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 5 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }

    async fn set_weapon_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> Option<Option<Weapon>> {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            "Back\n\
             (none)\n\
             Short Sword\n\
             Long Sword\n\
             Duelist Sword\n\
             Valor Blade",
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return None;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        None
                    }
                    1 => Some(None),
                    s => Some(Some(match s {
                        2 => Weapon {
                            name: String::from("Short Sword"),
                            attack: 2,
                        },
                        3 => Weapon {
                            name: String::from("Long Sword"),
                            attack: 7,
                        },
                        4 => Weapon {
                            name: String::from("Duelist Sword"),
                            attack: 13,
                        },
                        5 => Weapon {
                            name: String::from("Valor Blade"),
                            attack: 25,
                        },
                        _ => panic!("invalid weapon selection"),
                    })),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 5;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 5 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }

    async fn step_counts_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> bool {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            &format!(
                "Back\n\
                 Reset steps:{:>9}\n\
                 Town:{:>16}\n\
                 Wilderness 1:{:>8}\n\
                 Wilderness 2:{:>8}\n\
                 Wilderness 3:{:>8}\n\
                 Earth Castle:{:>8}\n\
                 Water Castle:{:>8}\n\
                 Fire Castle:{:>9}\n\n\
                 Next encounter:{:>6}",
                mctx.progress
                    .steps
                    .iter()
                    .copied()
                    .reduce(i32::saturating_add)
                    .unwrap_or(0),
                mctx.progress.steps.last().copied().unwrap_or(0),
                mctx.progress.steps[EncounterGroup::Wilderness1 as usize],
                mctx.progress.steps[EncounterGroup::Wilderness2 as usize],
                mctx.progress.steps[EncounterGroup::Wilderness3 as usize],
                mctx.progress.steps[EncounterGroup::EarthCastle as usize],
                mctx.progress.steps[EncounterGroup::WaterCastle as usize],
                mctx.progress.steps[EncounterGroup::FireCastle as usize],
                mctx.encounter_steps,
            ),
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return false;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        false
                    }
                    1 => true,
                    _ => unreachable!(),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up)
                || mctx.input.is_key_pressed(GameKey::Down)
            {
                mctx.audio.play_sfx(Sfx::Cursor);
                selection ^= 1;
                self.update_cursor_pos(selection);
            }
        }
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> DebugMenuEvent {
        const DEBUG_MENU_TEXT: &str = "Return\n\
             Gain Level\n\
             Battle…\n\
             Set Weapon…\n\
             Set Armor…\n\
             Get Items\n\
             Learn All Magic\n\
             Step Counts…\n\
             Flags…\n\
             Warp…\n\
             Quit Game";

        self.text.set_text(mctx.gctx, mctx.res, DEBUG_MENU_TEXT);

        let mut selection: i32 = 0;

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel)
                || mctx.input.is_key_pressed(GameKey::DebugMenu)
            {
                mctx.audio.play_sfx(Sfx::Cancel);
                return DebugMenuEvent::Cancel;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                let choice = match selection {
                    0 => Some(DebugMenuEvent::Cancel),
                    1 => Some(DebugMenuEvent::GainLevel),
                    2 => self.battle_menu(mctx).await.map(DebugMenuEvent::Battle),
                    3 => self
                        .set_weapon_menu(mctx)
                        .await
                        .map(DebugMenuEvent::SetWeapon),
                    4 => self
                        .set_armor_menu(mctx)
                        .await
                        .map(DebugMenuEvent::SetArmor),
                    5 => Some(DebugMenuEvent::GetItems),
                    6 => Some(DebugMenuEvent::LearnAllMagic),
                    7 => self
                        .step_counts_menu(mctx)
                        .await
                        .then_some(DebugMenuEvent::ResetStepCounts),
                    8 => self.flags_menu(mctx).await.map(DebugMenuEvent::ChangeFlag),
                    9 => self.warp_menu(mctx).await,
                    10 => Some(DebugMenuEvent::Quit),
                    _ => unreachable!(),
                };
                if let Some(choice) = choice {
                    mctx.audio.play_sfx(match choice {
                        DebugMenuEvent::Cancel => Sfx::Cancel,
                        _ => Sfx::Confirm,
                    });
                    return choice;
                }
                self.text.set_text(mctx.gctx, mctx.res, DEBUG_MENU_TEXT);
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 10;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 10 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }

    fn update_cursor_pos(&mut self, selection: i32) {
        self.cursor.set_offset(CURSOR_X, CURSOR_Y + selection * 8);
    }

    async fn warp_menu(&mut self, mctx: &mut ModeContext<'_, '_>) -> Option<DebugMenuEvent> {
        mctx.audio.play_sfx(Sfx::Confirm);

        self.text.set_text(
            mctx.gctx,
            mctx.res,
            "Back\n\
             Town\n\
             Earth Castle\n\
             Water Castle\n\
             Fire Castle",
        );

        let mut selection: i32 = 0;

        self.update_cursor_pos(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                return None;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                return match selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        None
                    }
                    s => Some(match s {
                        1 => DebugMenuEvent::Warp {
                            level_id: "Start",
                            x: 6,
                            y: 3,
                        },
                        2 => DebugMenuEvent::Warp {
                            level_id: "Earth_1",
                            x: 9,
                            y: 21,
                        },
                        3 => DebugMenuEvent::Warp {
                            level_id: "Water_1",
                            x: 9,
                            y: 21,
                        },
                        4 => DebugMenuEvent::Warp {
                            level_id: "Fire_1",
                            x: 9,
                            y: 21,
                        },
                        _ => panic!("invalid warp value"),
                    }),
                };
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 0 {
                    selection = 4;
                } else {
                    selection -= 1;
                }
                self.update_cursor_pos(selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if selection == 4 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.update_cursor_pos(selection);
            }
        }
    }
}
