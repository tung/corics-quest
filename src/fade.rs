use crate::async_utils::wait_once;

pub struct Fade([f32; 4]);

impl Fade {
    pub fn new() -> Self {
        Self([0.0; 4])
    }

    #[allow(clippy::wrong_self_convention)]
    pub async fn to_color(&mut self, frames: u16, target: [f32; 4]) {
        let start: [f32; 4] = self.0;
        for frame in 1..frames {
            let f = frame as f32 / frames as f32;
            self.0[0] = start[0] + (target[0] - start[0]) * f;
            self.0[1] = start[1] + (target[1] - start[1]) * f;
            self.0[2] = start[2] + (target[2] - start[2]) * f;
            self.0[3] = start[3] + (target[3] - start[3]) * f;
            wait_once().await;
        }
        self.0 = target;
    }

    pub async fn in_from_black(&mut self, frames: u16) {
        let target = [self.0[0], self.0[1], self.0[2], 0.0];
        self.0[3] = 1.0;
        self.to_color(frames, target).await;
    }

    pub async fn out_to_black(&mut self, frames: u16) {
        let target = [self.0[0], self.0[1], self.0[2], 1.0];
        self.0[3] = 0.0;
        self.to_color(frames, target).await;
    }

    pub fn set(&mut self, rgba: [f32; 4]) {
        self.0 = rgba;
    }
}

impl Default for Fade {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Fade> for [f32; 4] {
    fn from(fade: &Fade) -> Self {
        fade.0
    }
}
