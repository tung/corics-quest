use crate::progress::*;
use crate::random::*;

#[derive(Clone, Copy)]
pub enum EncounterGroup {
    Wilderness1,
    Wilderness2,
    Wilderness3,
    EarthCastle,
    WaterCastle,
    FireCastle,
}

#[derive(Clone)]
pub struct Enemy {
    pub name: &'static str,
    pub sprite_path: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub weakness: Option<Magic>,
    pub exp: i32,
}

const ENEMIES_WILDERNESS1: &[Enemy] = &[
    Enemy {
        name: "Rat",
        sprite_path: "rat.png",
        hp: 41,
        attack: 6,
        defense: 3,
        weakness: Some(Magic::FireEdge),
        exp: 5,
    },
    Enemy {
        name: "Dog",
        sprite_path: "dog.png",
        hp: 51,
        attack: 9,
        defense: 6,
        weakness: Some(Magic::FireEdge),
        exp: 5,
    },
];

const ENEMIES_WILDERNESS2: &[Enemy] = &[Enemy {
    name: "Rat Lv8",
    sprite_path: "rat.png",
    hp: 97,
    attack: 21,
    defense: 18,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

const ENEMIES_WILDERNESS3: &[Enemy] = &[Enemy {
    name: "Rat Lv16",
    sprite_path: "rat.png",
    hp: 172,
    attack: 41,
    defense: 38,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

const ENEMIES_EARTH_CASTLE: &[Enemy] = &[Enemy {
    name: "Rat Lv5",
    sprite_path: "rat.png",
    hp: 70,
    attack: 14,
    defense: 11,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

const ENEMIES_WATER_CASTLE: &[Enemy] = &[Enemy {
    name: "Rat Lv10",
    sprite_path: "rat.png",
    hp: 116,
    attack: 27,
    defense: 24,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

const ENEMIES_FIRE_CASTLE: &[Enemy] = &[Enemy {
    name: "Rat Lv20",
    sprite_path: "rat.png",
    hp: 210,
    attack: 52,
    defense: 49,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

impl EncounterGroup {
    pub fn random_enemy(self) -> Enemy {
        let group = match self {
            Self::Wilderness1 => ENEMIES_WILDERNESS1,
            Self::Wilderness2 => ENEMIES_WILDERNESS2,
            Self::Wilderness3 => ENEMIES_WILDERNESS3,
            Self::EarthCastle => ENEMIES_EARTH_CASTLE,
            Self::WaterCastle => ENEMIES_WATER_CASTLE,
            Self::FireCastle => ENEMIES_FIRE_CASTLE,
        };
        assert!(!group.is_empty());
        let i = u32::try_from(group.len()).expect("u32 enemy group length");
        group[random(i) as usize].clone()
    }
}

impl From<&str> for EncounterGroup {
    fn from(s: &str) -> Self {
        match s {
            "Wilderness1" => Self::Wilderness1,
            "Wilderness2" => Self::Wilderness2,
            "Wilderness3" => Self::Wilderness3,
            "EarthCastle" => Self::EarthCastle,
            "WaterCastle" => Self::WaterCastle,
            "FireCastle" => Self::FireCastle,
            _ => panic!("unknown encounter group: {s}"),
        }
    }
}
