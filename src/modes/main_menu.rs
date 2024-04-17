use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::input::*;
use crate::meter::*;
use crate::progress::*;
use crate::resources::*;
use crate::text::*;
use crate::window::*;

use miniquad::graphics::GraphicsContext;

const TOP_X: i32 = 47;
const TOP_Y: i32 = 25;
const TOP_WIDTH: i32 = 25 * 6 + 16;
const TOP_HEIGHT: i32 = 3 * 8 + 5 + 16;
const BOTTOM_X: i32 = TOP_X;
const BOTTOM_Y: i32 = TOP_Y + TOP_HEIGHT;
const BOTTOM_WIDTH: i32 = TOP_WIDTH;
const BOTTOM_HEIGHT: i32 = 8 * 8 + 16;
const MENU_X: i32 = TOP_X + TOP_WIDTH;
const MENU_Y: i32 = TOP_Y;
const MENU_WIDTH: i32 = 60;
const MENU_HEIGHT: i32 = 56;

pub struct MainMenu {
    top_window: Window,
    name_text: Text,
    level_text: Text,
    exp_meter: Meter,
    hp_text: Text,
    hp_meter: Meter,
    mp_text: Text,
    mp_meter: Meter,
    bottom_window: Window,
    bottom_text: Text,
    bottom_line: Text,
    bottom_cursor: Text,
    bottom_cursor_visible: bool,
    menu_window: Window,
    menu_text: Text,
    menu_cursor: Text,
}

pub enum MainMenuEvent {
    Done,
}

impl MainMenu {
    pub fn new(gctx: &mut GraphicsContext, res: &Resources, progress: &Progress) -> Self {
        let top_window = Window::new(gctx, res, TOP_X, TOP_Y, TOP_WIDTH, TOP_HEIGHT);
        let name_text = Text::from_str(
            gctx,
            res,
            TOP_X + 8,
            TOP_Y + 8,
            &format!(
                "Coric\n {:^11}",
                if progress.level <= 7 {
                    "Fighter"
                } else if progress.level <= 14 {
                    "Warrior"
                } else if progress.level <= 21 {
                    "Knight"
                } else if progress.level <= 29 {
                    "Valor Guard"
                } else {
                    "Blademaster"
                },
            ),
        );
        let level_text = Text::from_str(
            gctx,
            res,
            TOP_X + 8,
            TOP_Y + 8 + 2 * 8,
            &format!("Level {}", progress.level),
        );
        let mut exp_meter = Meter::new(
            gctx,
            res,
            TOP_X + 8,
            TOP_Y + 8 + 3 * 8 + 1,
            12 * 6,
            [255, 128, 50],
            progress.next_exp,
        );
        exp_meter.set_value(
            gctx,
            if progress.next_exp > 0 {
                progress.exp
            } else {
                1
            },
        );
        let hp_text = Text::from_str(
            gctx,
            res,
            TOP_X + 8 + 13 * 6,
            TOP_Y + 8,
            &format!("HP {:>3} / {:>3}", progress.hp, progress.max_hp),
        );
        let hp_meter = Meter::new(
            gctx,
            res,
            TOP_X + 8 + 13 * 6,
            TOP_Y + 8 + 8 + 1,
            12 * 6,
            [0, 192, 0],
            progress.max_hp,
        );
        let mp_text = Text::from_str(
            gctx,
            res,
            TOP_X + 8 + 13 * 6,
            TOP_Y + 8 + 2 * 8,
            &format!("MP {:>3} / {:>3}", progress.mp, progress.max_mp),
        );
        let mp_meter = Meter::new(
            gctx,
            res,
            TOP_X + 8 + 13 * 6,
            TOP_Y + 8 + 3 * 8 + 1,
            12 * 6,
            [0, 192, 192],
            progress.max_mp,
        );

        let bottom_window = Window::new(gctx, res, BOTTOM_X, BOTTOM_Y, BOTTOM_WIDTH, BOTTOM_HEIGHT);
        let bottom_text = Text::new(res, BOTTOM_X + 8, BOTTOM_Y + 8);
        let bottom_line = Text::new(res, BOTTOM_X + 8 + 6, BOTTOM_Y + BOTTOM_HEIGHT - 8 - 8);
        let bottom_cursor = Text::from_str(gctx, res, BOTTOM_X + 8, BOTTOM_Y + 8, "►");

        let menu_window = Window::new(gctx, res, MENU_X, MENU_Y, MENU_WIDTH, MENU_HEIGHT);
        let menu_text = Text::from_str(
            gctx,
            res,
            MENU_X + 14,
            MENU_Y + 8,
            "Return\n\nMagic\n\nItem",
        );
        let menu_cursor = Text::from_str(gctx, res, MENU_X + 8, MENU_Y + 8, "►");

        Self {
            top_window,
            name_text,
            level_text,
            exp_meter,
            hp_text,
            hp_meter,
            mp_text,
            mp_meter,
            bottom_window,
            bottom_text,
            bottom_line,
            bottom_cursor,
            bottom_cursor_visible: false,
            menu_window,
            menu_text,
            menu_cursor,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.top_window.draw(dctx.gctx);
        self.name_text.draw(dctx.gctx);
        self.level_text.draw(dctx.gctx);
        self.exp_meter.draw(dctx.gctx);
        self.hp_text.draw(dctx.gctx);
        self.hp_meter.draw(dctx.gctx);
        self.mp_text.draw(dctx.gctx);
        self.mp_meter.draw(dctx.gctx);

        self.bottom_window.draw(dctx.gctx);
        self.bottom_text.draw(dctx.gctx);
        self.bottom_line.draw(dctx.gctx);
        if self.bottom_cursor_visible {
            self.bottom_cursor.draw(dctx.gctx);
        }

        self.menu_window.draw(dctx.gctx);
        self.menu_text.draw(dctx.gctx);
        self.menu_cursor.draw(dctx.gctx);
    }

    async fn item_menu(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.update_bottom_text_for_item_menu(mctx);

        let mut selection = 0;

        self.bottom_cursor_visible = true;
        self.place_bottom_cursor(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                return;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                if selection == 0 {
                    return;
                } else {
                    let choice = usize::try_from(selection - 1).expect("selection - 1 as usize");
                    if mctx.progress.items[choice].amount > 0 {
                        match mctx.progress.items[choice].item {
                            Item::Salve => {
                                if mctx.progress.hp < mctx.progress.max_hp {
                                    mctx.progress.items[choice].amount -= 1;
                                    let heal_hp = (mctx.progress.max_hp * 3 + 9) / 10;
                                    mctx.progress.hp =
                                        mctx.progress.max_hp.min(mctx.progress.hp + heal_hp);
                                }
                            }
                            Item::XSalve => {
                                if mctx.progress.hp < mctx.progress.max_hp {
                                    mctx.progress.items[choice].amount -= 1;
                                    mctx.progress.hp = mctx.progress.max_hp;
                                }
                            }
                            Item::Tonic => {
                                if mctx.progress.mp < mctx.progress.max_mp {
                                    mctx.progress.items[choice].amount -= 1;
                                    let heal_mp = (mctx.progress.max_mp * 3 + 9) / 10;
                                    mctx.progress.mp =
                                        mctx.progress.max_mp.min(mctx.progress.mp + heal_mp);
                                }
                            }
                            Item::XTonic => {
                                if mctx.progress.mp < mctx.progress.max_mp {
                                    mctx.progress.items[choice].amount -= 1;
                                    mctx.progress.mp = mctx.progress.max_mp;
                                }
                            }
                        }
                        self.update_hp_and_mp(mctx);
                        self.update_bottom_text_for_item_menu(mctx);
                        self.update_bottom_line_for_item_menu(mctx, selection);
                    }
                }
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                if selection == 0 {
                    selection = 4;
                } else {
                    selection -= 1;
                }
                self.place_bottom_cursor(selection);
                self.update_bottom_line_for_item_menu(mctx, selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                if selection == 4 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.place_bottom_cursor(selection);
                self.update_bottom_line_for_item_menu(mctx, selection);
            }
        }
    }

    async fn magic_menu(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.update_bottom_text_for_magic_menu(mctx);

        let mut selection = 0;

        self.bottom_cursor_visible = true;
        self.place_bottom_cursor(selection);

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                return;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                if selection == 0 {
                    return;
                } else {
                    let choice = usize::try_from(selection - 1).expect("selection - 1 as usize");
                    let magic_slot = &mctx.progress.magic[choice];
                    if magic_slot.known
                        && matches!(magic_slot.magic, Magic::Heal)
                        && mctx.progress.mp >= magic_slot.magic.mp_cost()
                        && mctx.progress.hp < mctx.progress.max_hp
                    {
                        mctx.progress.mp -= magic_slot.magic.mp_cost();
                        let heal_amount = (mctx.progress.max_hp + 1) / 2;
                        mctx.progress.hp = mctx.progress.max_hp.min(mctx.progress.hp + heal_amount);
                        self.update_hp_and_mp(mctx);
                    }
                }
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                if selection == 0 {
                    selection = 4;
                } else {
                    selection -= 1;
                }
                self.place_bottom_cursor(selection);
                self.update_bottom_line_for_magic_menu(mctx, selection);
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                if selection == 4 {
                    selection = 0;
                } else {
                    selection += 1;
                }
                self.place_bottom_cursor(selection);
                self.update_bottom_line_for_magic_menu(mctx, selection);
            }
        }
    }

    fn place_bottom_cursor(&mut self, selection: i32) {
        self.bottom_cursor.set_offset(
            BOTTOM_X + 8,
            BOTTOM_Y
                + 8
                + if selection == 0 {
                    0
                } else {
                    (selection + 1) * 8
                },
        );
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> MainMenuEvent {
        self.update_hp_and_mp(mctx);
        self.update_bottom_text_for_status(mctx);

        let mut selection = 0;

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                return MainMenuEvent::Done;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                match selection {
                    0 => return MainMenuEvent::Done,
                    1 => self.magic_menu(mctx).await,
                    2 => self.item_menu(mctx).await,
                    _ => unreachable!(),
                }
                self.update_bottom_text_for_status(mctx);
                self.bottom_line.set_text(mctx.gctx, mctx.res, "");
                self.bottom_cursor_visible = false;
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                if selection == 0 {
                    selection = 2;
                } else {
                    selection -= 1;
                }
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                if selection == 2 {
                    selection = 0;
                } else {
                    selection += 1;
                }
            }
            self.menu_cursor
                .set_offset(MENU_X + 8, MENU_Y + 8 + 16 * selection);
        }
    }

    fn update_bottom_line_for_item_menu(&mut self, mctx: &mut ModeContext<'_, '_>, selection: i32) {
        self.bottom_line.set_text(
            mctx.gctx,
            mctx.res,
            if selection == 0 {
                ""
            } else {
                let choice = usize::try_from(selection - 1).expect("selection - 1 as usize");
                mctx.progress.items[choice].description()
            },
        );
    }

    fn update_bottom_line_for_magic_menu(
        &mut self,
        mctx: &mut ModeContext<'_, '_>,
        selection: i32,
    ) {
        self.bottom_line.set_text(
            mctx.gctx,
            mctx.res,
            if selection == 0 {
                ""
            } else {
                let choice = usize::try_from(selection - 1).expect("selection - 1 as usize");
                mctx.progress.magic[choice].description()
            },
        );
    }

    fn update_bottom_text_for_item_menu(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.bottom_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!(
                " Back\n\n {:23.23}\n {:23.23}\n {:23.23}\n {:23.23}",
                mctx.progress.items[0].main_menu_entry(),
                mctx.progress.items[1].main_menu_entry(),
                mctx.progress.items[2].main_menu_entry(),
                mctx.progress.items[3].main_menu_entry(),
            ),
        );
    }

    fn update_bottom_text_for_magic_menu(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.bottom_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!(
                " Back\n\n {:23.23}\n {:23.23}\n {:23.23}\n {:23.23}",
                mctx.progress.magic[0].main_menu_entry(),
                mctx.progress.magic[1].main_menu_entry(),
                mctx.progress.magic[2].main_menu_entry(),
                mctx.progress.magic[3].main_menu_entry(),
            ),
        );
    }

    fn update_bottom_text_for_status(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.bottom_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!(
                "    Weapon:{:>14}\
               \n    Attack:{:>14}\
             \n\n     Armor:{:>14}\
               \n   Defense:{:>14}\
             \n\nExperience:{:>14}\
               \nNext Level:{:>14}",
                mctx.progress
                    .weapon
                    .as_ref()
                    .map(|w| w.name.as_str())
                    .unwrap_or("(none)"),
                mctx.progress.attack,
                mctx.progress
                    .armor
                    .as_ref()
                    .map(|a| a.name.as_str())
                    .unwrap_or("(none)"),
                mctx.progress.defense,
                mctx.progress.exp,
                mctx.progress.next_exp,
            ),
        );
    }

    fn update_hp_and_mp(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.hp_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!("HP {:>3} / {:>3}", mctx.progress.hp, mctx.progress.max_hp),
        );
        self.hp_meter.set_value(mctx.gctx, mctx.progress.hp);
        self.mp_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!("MP {:>3} / {:>3}", mctx.progress.mp, mctx.progress.max_mp),
        );
        self.mp_meter.set_value(mctx.gctx, mctx.progress.mp);
    }
}
