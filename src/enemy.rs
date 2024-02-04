use crate::progress::*;
use crate::random::*;

#[derive(Clone, Copy)]
pub enum EncounterGroup {
    Forest1,
}

pub struct Enemy {
    pub name: &'static str,
    pub sprite_path: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub weakness: Option<Magic>,
}

impl EncounterGroup {
    pub fn random_enemy(self) -> Enemy {
        let Self::Forest1 = self;
        let r = random(2);
        if r == 0 {
            Enemy {
                name: "Rat",
                sprite_path: "rat.png",
                hp: 52,
                attack: 5,
                defense: 5,
                weakness: Some(Magic::FireEdge),
            }
        } else {
            Enemy {
                name: "Dog",
                sprite_path: "dog.png",
                hp: 52,
                attack: 5,
                defense: 5,
                weakness: Some(Magic::FireEdge),
            }
        }
    }
}

impl From<&str> for EncounterGroup {
    fn from(s: &str) -> Self {
        match s {
            "Forest1" => Self::Forest1,
            _ => panic!("unknown encounter group: {s}"),
        }
    }
}
