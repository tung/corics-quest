use crate::async_utils::wait_once;
use crate::contexts::*;
use crate::enemy::*;
use crate::input::*;
use crate::resources::*;
use crate::sprite::*;
use crate::text::*;
use crate::window::*;

use miniquad::graphics::GraphicsContext;

const ENEMY_X: i32 = 104;
const ENEMY_Y: i32 = 32;
const MESSAGE_X: i32 = 18;
const MESSAGE_Y: i32 = 116;
const STATUS_X: i32 = 250;
const STATUS_Y: i32 = 116;

pub struct Battle {
    enemy_window: Window,
    enemy_sprite: Sprite,
    enemy_visible: bool,
    message_window: Window,
    message_text: Text,
    menu_text: Text,
    cursor: Text,
    menu_visible: bool,
    status_window: Window,
    status_visible: bool,
    hp_text: Text,
    mp_text: Text,
    change_window: Window,
    change_text: Text,
    change_visible: bool,
    enemy: Enemy,
}

pub enum BattleEvent {
    Defeat,
    RanAway,
    Victory,
}

enum PlayerChoice {
    Fight,
    Run,
}

impl Battle {
    pub fn new(gctx: &mut GraphicsContext, res: &Resources, enemy: Enemy) -> Self {
        let mut enemy_sprite = Sprite::new(gctx, res, enemy.sprite_path);
        enemy_sprite.start_animation("idle");

        Self {
            enemy_window: Window::new(gctx, res, ENEMY_X, ENEMY_Y, 112, 80),
            enemy_sprite,
            enemy_visible: true,
            message_window: Window::new(gctx, res, MESSAGE_X, MESSAGE_Y, 232, 48),
            message_text: Text::new(res, MESSAGE_X + 8, MESSAGE_Y + 8),
            menu_text: Text::new(res, MESSAGE_X + 8, MESSAGE_Y + 24),
            cursor: Text::from_str(gctx, res, MESSAGE_X + 8, MESSAGE_Y + 32, "►"),
            menu_visible: false,
            status_window: Window::new(gctx, res, STATUS_X, STATUS_Y, 52, 48),
            status_visible: true,
            hp_text: Text::new(res, STATUS_X + 8, STATUS_Y + 8),
            mp_text: Text::new(res, STATUS_X + 8, STATUS_Y + 24),
            change_window: Window::new(gctx, res, 0, 0, 16, 24),
            change_text: Text::new(res, 0, 0),
            change_visible: false,
            enemy,
        }
    }

    async fn action_menu(
        &mut self,
        mctx: &mut ModeContext<'_, '_>,
        has_follow_up: bool,
    ) -> PlayerChoice {
        fn update_action_cursor(cursor: &mut Text, which: usize) {
            //           1         2         3
            // 012345678901234567890123456789012345
            // .Fight     .Magic    .Item    .Run
            const ACTION_POSITIONS: [i32; 4] = [0, 11, 21, 30];
            let x = ACTION_POSITIONS[which] * 6;
            let y = 8;
            cursor.set_offset(MESSAGE_X + 8 + x, MESSAGE_Y + 24 + y);
        }

        fn set_all_text(
            mctx: &mut ModeContext<'_, '_>,
            message_text: &mut Text,
            menu_text: &mut Text,
            enemy_name: &str,
            has_follow_up: bool,
        ) {
            message_text.set_text(
                mctx.gctx,
                mctx.res,
                &format!(
                    "{} prepares to fight!\nChoose your course of action:",
                    enemy_name,
                ),
            );

            menu_text.set_text(
                mctx.gctx,
                mctx.res,
                &format!(
                    "\n {:8}   Magic     Item     Run",
                    if has_follow_up { "FollowUp" } else { "Fight" },
                ),
            );
        }

        set_all_text(
            mctx,
            &mut self.message_text,
            &mut self.menu_text,
            self.enemy.name,
            has_follow_up,
        );
        self.message_text.reveal().await;
        self.menu_visible = true;

        let mut selection = 0;
        update_action_cursor(&mut self.cursor, selection);

        loop {
            self.enemy_sprite.animate();

            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Confirm) {
                let choice = match selection {
                    0 => Some(PlayerChoice::Fight),
                    3 => Some(PlayerChoice::Run),
                    _ => None,
                };
                if let Some(choice) = choice {
                    self.menu_visible = false;
                    return choice;
                } else {
                    set_all_text(
                        mctx,
                        &mut self.message_text,
                        &mut self.menu_text,
                        self.enemy.name,
                        has_follow_up,
                    );
                    update_action_cursor(&mut self.cursor, selection);
                }
            } else if mctx.input.is_key_pressed(GameKey::Left) {
                match selection {
                    0 => selection = 3,
                    _ => selection -= 1,
                }
                update_action_cursor(&mut self.cursor, selection);
            } else if mctx.input.is_key_pressed(GameKey::Right) {
                match selection {
                    3 => selection = 0,
                    _ => selection += 1,
                }
                update_action_cursor(&mut self.cursor, selection);
            }
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.enemy_window.draw(dctx.gctx);
        if self.enemy_visible {
            self.enemy_sprite
                .draw(dctx.gctx, ENEMY_X + 40, ENEMY_Y + 24);
        }
        self.message_window.draw(dctx.gctx);
        self.message_text.draw(dctx.gctx);
        if self.menu_visible {
            self.menu_text.draw(dctx.gctx);
            self.cursor.draw(dctx.gctx);
        }
        if self.status_visible {
            self.status_window.draw(dctx.gctx);
            self.hp_text.draw(dctx.gctx);
            self.mp_text.draw(dctx.gctx);
        }
        if self.change_visible {
            self.change_window.draw(dctx.gctx);
            self.change_text.draw(dctx.gctx);
        }
    }

    async fn enemy_hit_animation(&mut self, mctx: &mut ModeContext<'_, '_>, damage: i32) {
        for _ in 0..5 {
            self.enemy_visible = false;
            wait_once().await;
            wait_once().await;
            self.enemy_visible = true;
            wait_once().await;
            wait_once().await;
        }
        self.show_change_text_at(mctx, ENEMY_X + 56, ENEMY_Y + 16, &format!("{damage}"));
    }

    fn show_change_text_at(
        &mut self,
        mctx: &mut ModeContext<'_, '_>,
        middle_x: i32,
        bottom_y: i32,
        msg: &str,
    ) {
        self.change_text.set_text(mctx.gctx, mctx.res, msg);
        let text_width = self.change_text.width();
        let text_height = self.change_text.height();
        self.change_text
            .set_offset(middle_x - text_width / 2, bottom_y - (text_height + 6));
        self.change_window
            .resize(mctx.gctx, text_width + 12, text_height + 12);
        self.change_window.set_offset(
            middle_x - (text_width + 12) / 2,
            bottom_y - (text_height + 12),
        );
        self.change_visible = true;
    }

    fn show_status_change(&mut self, mctx: &mut ModeContext<'_, '_>, msg: &str) {
        self.show_change_text_at(mctx, STATUS_X + 26, STATUS_Y + 6, msg);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> BattleEvent {
        self.update_status(mctx);

        loop {
            match self.action_menu(mctx, false).await {
                PlayerChoice::Fight => {
                    let damage =
                        calc_base_damage(mctx.progress.attack, self.enemy.defense).trunc() as i32;
                    self.enemy_hit_animation(mctx, damage).await;
                    self.enemy.hp -= damage.min(self.enemy.hp);

                    self.message_text.set_text(
                        mctx.gctx,
                        mctx.res,
                        &format!("Coric attacks!\n{damage} damage to {}", self.enemy.name),
                    );
                    self.message_text.reveal().await;
                    self.wait_for_confirmation(mctx).await;
                }

                PlayerChoice::Run => {
                    self.message_text
                        .set_text(mctx.gctx, mctx.res, "Coric ran away!");
                    self.message_text.reveal().await;
                    self.wait_for_confirmation(mctx).await;
                    return BattleEvent::RanAway;
                }
            }

            self.update_status(mctx);

            if self.enemy.hp <= 0 {
                self.enemy_visible = false;

                if mctx.progress.mp < mctx.progress.max_mp {
                    self.show_status_change(mctx, "+1MP");
                    mctx.progress.mp += 1;
                    self.update_status(mctx);
                }

                self.message_text.set_text(
                    mctx.gctx,
                    mctx.res,
                    &format!("{} is defeated!", self.enemy.name),
                );
                self.message_text.reveal().await;
                self.wait_for_confirmation(mctx).await;
                return BattleEvent::Victory;
            }

            let base_damage = calc_base_damage(self.enemy.attack, mctx.progress.defense);
            let damage = base_damage.trunc() as i32;
            for _ in 0..5 {
                self.status_visible = false;
                wait_once().await;
                wait_once().await;
                self.status_visible = true;
                wait_once().await;
                wait_once().await;
            }

            self.show_status_change(mctx, &format!("{damage}"));
            mctx.progress.hp -= damage.min(mctx.progress.hp);
            self.update_status(mctx);

            self.message_text.set_text(
                mctx.gctx,
                mctx.res,
                &format!("{} attacks!\n{damage} HP damage to Coric.", self.enemy.name),
            );
            self.message_text.reveal().await;
            self.wait_for_confirmation(mctx).await;

            if mctx.progress.hp <= 0 {
                return BattleEvent::Defeat;
            }
        }
    }

    fn update_status(&mut self, mctx: &mut ModeContext<'_, '_>) {
        self.hp_text
            .set_text(mctx.gctx, mctx.res, &format!("HP{:4}", mctx.progress.hp));
        self.mp_text
            .set_text(mctx.gctx, mctx.res, &format!("MP{:4}", mctx.progress.mp));
    }

    async fn wait_for_confirmation(&mut self, mctx: &mut ModeContext<'_, '_>) {
        while !mctx.input.is_key_pressed(GameKey::Confirm) {
            self.enemy_sprite.animate();
            wait_once().await;
        }
        self.change_visible = false;
    }
}

fn calc_base_damage(attack: i32, defense: i32) -> f32 {
    let attack = attack as f32;
    let defense = defense as f32;
    if attack * attack < defense {
        1.0
    } else if attack < defense {
        attack * attack / defense
    } else {
        attack * 2.0 - defense
    }
}
