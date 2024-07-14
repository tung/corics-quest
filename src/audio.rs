#[cfg(target_arch = "wasm32")]
use crate::async_utils::wait_once;

use quad_snd::{AudioContext, PlaySoundParams, Sound};

use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Music {
    Battle,
    Boss,
    Dungeon,
    Ending,
    Intro,
    Overworld,
    Town,
}

#[derive(Clone, Copy)]
pub enum Sfx {
    Attack,
    Cancel,
    Chime,
    Confirm,
    Cursor,
    Heal,
    Hurt,
    Magic,
}

pub struct Audio {
    audio_context: AudioContext,
    music: Option<(Music, Sound)>,
    music_volume_custom: u8,
    music_volume_scripted: u8,
    music_volume_scripted_target: u8,
    sound_effects: [Sound; Sfx::NUM_SFXS],
}

pub const MAX_MUSIC_VOLUME: u8 = 100;

#[rustfmt::skip]
const SFX_SOUND_DATA: [&[u8]; Sfx::NUM_SFXS] = [
    include_bytes!("../assets/attack.ogg"), // Sfx::Attack
    include_bytes!("../assets/cancel.ogg"), // Sfx::Cancel
    include_bytes!("../assets/chime.ogg"), // Sfx::Chime
    include_bytes!("../assets/confirm.ogg"), // Sfx::Confirm
    include_bytes!("../assets/cursor.ogg"), // Sfx::Cursor
    include_bytes!("../assets/heal.ogg"), // Sfx::Heal
    include_bytes!("../assets/hurt.ogg"), // Sfx::Hurt
    include_bytes!("../assets/magic.ogg"), // Sfx::Magic
];

impl Music {
    fn sound_data(&self) -> &'static [u8] {
        match self {
            Self::Battle => include_bytes!("../assets/battle.ogg"),
            Self::Boss => include_bytes!("../assets/boss.ogg"),
            Self::Dungeon => include_bytes!("../assets/dungeon.ogg"),
            Self::Ending => include_bytes!("../assets/ending.ogg"),
            Self::Intro => include_bytes!("../assets/intro.ogg"),
            Self::Overworld => include_bytes!("../assets/overworld.ogg"),
            Self::Town => include_bytes!("../assets/town.ogg"),
        }
    }

    fn base_volume(&self) -> f32 {
        match self {
            Self::Boss => 0.544,
            _ => 1.0,
        }
    }
}

impl From<&str> for Music {
    fn from(s: &str) -> Self {
        match s {
            "Battle" => Self::Battle,
            "Boss" => Self::Boss,
            "Dungeon" => Self::Dungeon,
            "Ending" => Self::Ending,
            "Intro" => Self::Intro,
            "Overworld" => Self::Overworld,
            "Town" => Self::Town,
            _ => panic!("unknown music: {s}"),
        }
    }
}

impl Sfx {
    const NUM_SFXS: usize = Self::Magic as usize + 1;

    fn base_volume(&self) -> f32 {
        match self {
            Sfx::Cursor => 0.8,
            Sfx::Chime => 0.6,
            Sfx::Heal => 0.5,
            Sfx::Hurt => 0.5,
            Sfx::Magic => 0.6,
            _ => 1.0,
        }
    }
}

impl Audio {
    pub fn new() -> Self {
        let audio_context = AudioContext::new();
        let sound_effects =
            core::array::from_fn(|i| Sound::load(&audio_context, SFX_SOUND_DATA[i]));

        Self {
            audio_context,
            music: None,
            music_volume_custom: MAX_MUSIC_VOLUME,
            music_volume_scripted: MAX_MUSIC_VOLUME,
            music_volume_scripted_target: MAX_MUSIC_VOLUME,
            sound_effects,
        }
    }

    pub fn adjust_music_volume_scripted(&mut self) {
        match self
            .music_volume_scripted
            .cmp(&self.music_volume_scripted_target)
        {
            Ordering::Less => self.music_volume_scripted += 1,
            Ordering::Greater => self.music_volume_scripted -= 1,
            _ => return,
        }
        if let Some((music, sound)) = &self.music {
            sound.set_volume(&self.audio_context, self.calc_music_volume(*music));
        }
    }

    fn calc_music_volume(&self, music: Music) -> f32 {
        music.base_volume()
            * (self.music_volume_custom as f32 / MAX_MUSIC_VOLUME as f32)
            * (self.music_volume_scripted as f32 / MAX_MUSIC_VOLUME as f32)
    }

    pub fn get_music_volume_custom(&self) -> u8 {
        self.music_volume_custom
    }

    pub async fn play_music(&mut self, music: Option<Music>) {
        if let Some((current, sound)) = &self.music {
            if music.map(|m| m != *current).unwrap_or(true) {
                sound.stop(&self.audio_context);
                sound.delete(&self.audio_context);
                self.music = None;
            }
        }

        if let Some(music) = music {
            if let Some((current_music, _)) = &self.music {
                if music == *current_music {
                    return;
                }
            }

            let sound = Sound::load(&self.audio_context, music.sound_data());
            #[cfg(target_arch = "wasm32")]
            while !sound.is_loaded() {
                wait_once().await;
            }

            sound.play(
                &self.audio_context,
                PlaySoundParams {
                    looped: true,
                    volume: self.calc_music_volume(music),
                },
            );
            self.music = Some((music, sound));
        }
    }

    pub fn play_sfx(&self, sfx: Sfx) {
        let is_cursor_sound = matches!(sfx, Sfx::Cancel | Sfx::Chime | Sfx::Confirm | Sfx::Cursor);
        if is_cursor_sound {
            // Don't let cursor sounds stack on top of each other.
            for s in [Sfx::Cancel, Sfx::Confirm, Sfx::Cursor] {
                self.sound_effects[s as usize].stop(&self.audio_context);
            }
        }
        let sound = &self.sound_effects[sfx as usize];
        if !is_cursor_sound {
            // Also stop non-cursor sounds from stacking on top of themselves.
            sound.stop(&self.audio_context);
        }
        sound.play(
            &self.audio_context,
            PlaySoundParams {
                looped: false,
                volume: sfx.base_volume(),
            },
        );
    }

    pub fn set_music_volume_custom(&mut self, volume: u8) {
        self.music_volume_custom = volume.min(MAX_MUSIC_VOLUME);
        if let Some((music, sound)) = &self.music {
            sound.set_volume(&self.audio_context, self.calc_music_volume(*music));
        }
    }

    pub fn set_music_volume_scripted(&mut self, volume: u8) {
        assert!(volume <= MAX_MUSIC_VOLUME);
        self.music_volume_scripted_target = volume;
    }
}
