use crate::async_utils::wait_once;
use crate::audio::*;
use crate::contexts::*;
use crate::input::*;
use crate::meter::*;
use crate::resources::*;
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
}

pub enum OptionsEvent {
    Done,
}

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
        }
    }

    pub fn draw(&self, dctx: &mut DrawContext) {
        self.cursor.draw(dctx.gctx);
        self.back_text.draw(dctx.gctx);
        self.music_text.draw(dctx.gctx);
        self.music_meter.draw(dctx.gctx);
        self.sound_text.draw(dctx.gctx);
        self.sound_meter.draw(dctx.gctx);
    }

    pub async fn update(&mut self, mctx: &mut ModeContext<'_, '_>) -> OptionsEvent {
        self.update_volumes(mctx);
        self.update_cursor_pos();

        loop {
            wait_once().await;

            if mctx.input.is_key_pressed(GameKey::Cancel) {
                mctx.audio.play_sfx(Sfx::Cancel);
                if self.preview_music {
                    mctx.audio.play_music(None).await;
                }
                return OptionsEvent::Done;
            } else if mctx.input.is_key_pressed(GameKey::Confirm) {
                if self.selection == 0 {
                    mctx.audio.play_sfx(Sfx::Cancel);
                    return OptionsEvent::Done;
                }
            } else if mctx.input.is_key_pressed(GameKey::Up) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == 0 {
                    self.selection = 2;
                } else {
                    self.selection -= 1;
                }
                self.update_cursor_pos();
                if self.preview_music {
                    mctx.audio.play_music(None).await;
                }
            } else if mctx.input.is_key_pressed(GameKey::Down) {
                mctx.audio.play_sfx(Sfx::Cursor);
                if self.selection == 2 {
                    self.selection = 0;
                } else {
                    self.selection += 1;
                }
                self.update_cursor_pos();
                if self.preview_music {
                    mctx.audio.play_music(None).await;
                }
            } else if self.selection == 1 {
                if mctx.input.is_key_pressed(GameKey::Left) {
                    let new_volume = mctx.audio.get_music_volume_custom().saturating_sub(10);
                    mctx.audio.set_music_volume_custom(new_volume);
                    self.update_volumes(mctx);
                    if self.preview_music {
                        mctx.audio.play_music(Some(Music::Overworld)).await;
                    }
                } else if mctx.input.is_key_pressed(GameKey::Right) {
                    let new_volume = mctx.audio.get_music_volume_custom().saturating_add(10);
                    mctx.audio.set_music_volume_custom(new_volume);
                    self.update_volumes(mctx);
                    if self.preview_music {
                        mctx.audio.play_music(Some(Music::Overworld)).await;
                    }
                }
            } else if self.selection == 2 {
                if mctx.input.is_key_pressed(GameKey::Left) {
                    let new_volume = mctx.audio.get_sound_volume_custom().saturating_sub(10);
                    mctx.audio.set_sound_volume_custom(new_volume);
                    self.update_volumes(mctx);
                    mctx.audio.play_sfx(Sfx::Cursor);
                } else if mctx.input.is_key_pressed(GameKey::Right) {
                    let new_volume = mctx.audio.get_sound_volume_custom().saturating_add(10);
                    mctx.audio.set_sound_volume_custom(new_volume);
                    self.update_volumes(mctx);
                    mctx.audio.play_sfx(Sfx::Cursor);
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
