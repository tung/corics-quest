use miniquad::KeyCode;

#[derive(Clone, Copy)]
pub enum GameKey {
    DebugQuit,
    Up,
    Down,
    Left,
    Right,
}

pub struct Input {
    keys_down: Vec<bool>,
}

impl GameKey {
    const NUM_KEYS: usize = Self::Right as usize + 1;
}

impl TryFrom<KeyCode> for GameKey {
    type Error = ();

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyCode::Q => Self::DebugQuit,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            _ => return Err(()),
        })
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: vec![false; GameKey::NUM_KEYS],
        }
    }

    pub fn handle_key_down_event(&mut self, keycode: KeyCode) {
        if let Ok(game_key) = GameKey::try_from(keycode) {
            self.keys_down[game_key as usize] = true;
        }
    }

    pub fn handle_key_up_event(&mut self, keycode: KeyCode) {
        if let Ok(game_key) = GameKey::try_from(keycode) {
            self.keys_down[game_key as usize] = false;
        }
    }

    pub fn is_key_down(&self, game_key: GameKey) -> bool {
        self.keys_down[game_key as usize]
    }
}