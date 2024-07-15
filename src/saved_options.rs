use crate::audio::*;

use miniserde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SavedOptions {
    pub music_volume: u8,
    pub sound_volume: u8,
}

const OPTIONS_KEY: &str = "options";

impl SavedOptions {
    pub fn new() -> Self {
        Self {
            music_volume: MAX_MUSIC_VOLUME,
            sound_volume: MAX_SOUND_VOLUME,
        }
    }

    pub fn load() -> Result<Self, &'static str> {
        let raw_data = quad_storage::STORAGE
            .lock()
            .map_err(|_| "storage error")?
            .get(OPTIONS_KEY)
            .ok_or("no option file found")?;
        miniserde::json::from_str(&raw_data).map_err(|_| "failed to parse options data")
    }

    pub fn save(&self) -> Result<(), &'static str> {
        let raw_data = miniserde::json::to_string(self);
        quad_storage::STORAGE
            .lock()
            .map_err(|_| "storage error")?
            .set(OPTIONS_KEY, &raw_data);
        Ok(())
    }
}
