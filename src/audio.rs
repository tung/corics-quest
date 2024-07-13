#[cfg(target_arch = "wasm32")]
use crate::async_utils::wait_once;

use quad_snd::{AudioContext, PlaySoundParams, Sound};

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

pub struct Audio {
    audio_context: AudioContext,
    music: Option<(Music, Sound)>,
    music_volume_custom: u8,
    music_volume_scripted: u8,
}

pub const MAX_MUSIC_VOLUME: u8 = 100;

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

impl Audio {
    pub fn new() -> Self {
        Self {
            audio_context: AudioContext::new(),
            music: None,
            music_volume_custom: MAX_MUSIC_VOLUME,
            music_volume_scripted: MAX_MUSIC_VOLUME,
        }
    }

    fn calc_music_volume(&self) -> f32 {
        self.music
            .as_ref()
            .map(|(m, _)| m.base_volume())
            .unwrap_or(1.0)
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
                    volume: self.calc_music_volume(),
                },
            );
            self.music = Some((music, sound));
        }
    }

    pub fn set_music_volume_custom(&mut self, volume: u8) {
        self.music_volume_custom = volume.min(MAX_MUSIC_VOLUME);
        if let Some((_, sound)) = &self.music {
            sound.set_volume(&self.audio_context, self.calc_music_volume());
        }
    }

    pub fn set_music_volume_scripted(&mut self, volume: u8) {
        assert!(volume <= MAX_MUSIC_VOLUME);
        self.music_volume_scripted = volume;
        if let Some((_, sound)) = &self.music {
            sound.set_volume(&self.audio_context, self.calc_music_volume());
        }
    }
}
