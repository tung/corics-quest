use crate::progress::*;

pub struct Enemy {
    pub name: &'static str,
    pub sprite_path: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub weakness: Option<Magic>,
}
