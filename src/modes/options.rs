use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::meter::*;
use crate::resources::*;
use crate::saved_options::*;
use crate::text::*;

use miniquad::GlContext;

pub struct Options {
    base_x: i32,
    base_y: i32,
    preview_music: bool,
    selection: i32,
    cursor: Text,
    back_text: Text,
    music_text: Text,
    music_meter: Meter,
    sound_text: Text,
    sound_meter: Meter,
    credits_text: Text,
    quit_text: Text,
    options_changed: bool,
}

pub enum OptionsEvent {
    Credits,
    Done,
    Quit,
}

#[cfg(target_arch = "wasm32")]
const NUM_ENTRIES: i32 = 4;
#[cfg(not(target_arch = "wasm32"))]
const NUM_ENTRIES: i32 = 5;

#[cfg(target_arch = "wasm32")]
const QUIT_STR: &str = "";
#[cfg(not(target_arch = "wasm32"))]
const QUIT_STR: &str = "Quit";

impl Options {
    pub fn new(
        gctx: &mut GlContext,
        res: &Resources,
        base_x: i32,
        base_y: i32,
        preview_music: bool,
    ) -> Self {
        Self {
            base_x,
            base_y,
            preview_music,
            selection: 0,
            cursor: Text::from_str(gctx, res, 0, 0, "►"),
            back_text: Text::from_str(gctx, res, base_x + 6, base_y, "Back"),
            music_text: Text::new(res, base_x + 6, base_y + 2 * 8),
            music_meter: Meter::new(
                gctx,
                res,
                base_x + 13 * 6,
                base_y + 2 * 8 + 2,
                12 * 6,
                [192, 192, 192],
                MAX_MUSIC_VOLUME as i32,
            ),
            sound_text: Text::new(res, base_x + 6, base_y + 3 * 8),
            sound_meter: Meter::new(
                gctx,
                res,
                base_x + 13 * 6,
                base_y + 3 * 8 + 2,
                12 * 6,
                [192, 192, 192],
                MAX_SOUND_VOLUME as i32,
            ),
            credits_text: Text::from_str(gctx, res, base_x + 6, base_y + 4 * 8, "Credits"),
            quit_text: Text::from_str(gctx, res, base_x + 6, base_y + 5 * 8, QUIT_STR),
            options_changed: false,
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.cursor.draw(dctx.gctx);
        self.back_text.draw(dctx.gctx);
        self.music_text.draw(dctx.gctx);
        self.music_meter.draw(dctx.gctx);
        self.sound_text.draw(dctx.gctx);
        self.sound_meter.draw(dctx.gctx);
        self.credits_text.draw(dctx.gctx);
        self.quit_text.draw(dctx.gctx);
    }

    fn save_changed_options(&mut self, mctx: &mut ModeContext) {
        if self.options_changed {
            let opts = SavedOptions {
                music_volume: mctx.audio.get_music_volume_custom(),
                sound_volume: mctx.audio.get_sound_volume_custom(),
            };
            // Save options on a best-effort basis.
            let _ = opts.save();
            self.options_changed = false;
        }
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> OptionsEvent {
        self.update_volumes(mctx);
        self.update_cursor_pos();

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                self.save_changed_options(mctx);
                return OptionsEvent::Done;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                match self.selection {
                    0 => {
                        mctx.audio.play_sfx(Sfx::Cancel);
                        self.save_changed_options(mctx);
                        return OptionsEvent::Done;
                    }
                    1 => {
                        if self.preview_music {
                            mctx.audio.play_music(Some(Music::Overworld)).await;
                        }
                    }
                    2 => mctx.audio.play_sfx(Sfx::Confirm),
                    3 => return OptionsEvent::Credits,
                    4 => {
                        self.save_changed_options(mctx);
                        return OptionsEvent::Quit;
                    }
                    _ => unreachable!(),
                }
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == 0 {
                    self.selection = NUM_ENTRIES - 1;
                } else {
                    self.selection -= 1;
                }
                self.update_cursor_pos();
                if self.preview_music {
                    mctx.audio.play_music(None).await;
                }
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == NUM_ENTRIES - 1 {
                    self.selection = 0;
                } else {
                    self.selection += 1;
                }
                self.update_cursor_pos();
                if self.preview_music {
                    mctx.audio.play_music(None).await;
                }
            } else if self.selection == 1 {
                let old_volume = mctx.audio.get_music_volume_custom();
                let mut music_volume_changed = false;

                if mctx.input.is_key_pressed(GameKey::Left) {
                    mctx.audio
                        .set_music_volume_custom(old_volume.saturating_sub(10));
                    music_volume_changed = true;
                } else if mctx.input.is_key_pressed(GameKey::Right) {
                    let old_volume = mctx.audio.get_music_volume_custom();
                    mctx.audio
                        .set_music_volume_custom(old_volume.saturating_add(10));
                    music_volume_changed = true;
                }

                if music_volume_changed {
                    self.options_changed |= mctx.audio.get_music_volume_custom() != old_volume;
                    self.update_volumes(mctx);
                    if self.preview_music {
                        mctx.audio.play_music(Some(Music::Overworld)).await;
                    }
                }
            } else if self.selection == 2 {
                let old_volume = mctx.audio.get_sound_volume_custom();
                let mut sound_volume_changed = false;

                if mctx.input.is_key_pressed(GameKey::Left) {
                    mctx.audio
                        .set_sound_volume_custom(old_volume.saturating_sub(10));
                    sound_volume_changed = true;
                } else if mctx.input.is_key_pressed(GameKey::Right) {
                    mctx.audio
                        .set_sound_volume_custom(old_volume.saturating_add(10));
                    sound_volume_changed = true;
                }

                if sound_volume_changed {
                    self.options_changed |= mctx.audio.get_sound_volume_custom() != old_volume;
                    self.update_volumes(mctx);
                    mctx.audio.play_sfx(Sfx::Confirm);
                }
            }
        }
    }

    fn update_cursor_pos(&mut self) {
        let y = if self.selection == 0 {
            self.base_y
        } else {
            self.base_y + (self.selection + 1) * 8
        };
        self.cursor.set_offset(self.base_x, y);
    }

    fn update_volumes(&mut self, mctx: &mut ModeContext) {
        let music_volume_custom = mctx.audio.get_music_volume_custom();
        self.music_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!("Music{:>4}", music_volume_custom / 10),
        );
        self.music_meter
            .set_value(mctx.gctx, music_volume_custom as i32);

        let sound_volume_custom = mctx.audio.get_sound_volume_custom();
        self.sound_text.set_text(
            mctx.gctx,
            mctx.res,
            &format!("Sound{:>4}", sound_volume_custom / 10),
        );
        self.sound_meter
            .set_value(mctx.gctx, sound_volume_custom as i32);
    }
}
