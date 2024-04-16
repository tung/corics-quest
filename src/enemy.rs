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
    // level 1
    Enemy {
        name: "Rat",
        sprite_path: "rat.png",
        hp: 32,
        attack: 4,
        defense: 1,
        weakness: None,
        exp: 5,
    },
    // level 2
    Enemy {
        name: "Dog",
        sprite_path: "dog.png",
        hp: 42,
        attack: 6,
        defense: 3,
        weakness: None,
        exp: 5,
    },
    // level 2.5
    Enemy {
        name: "Horn Beast",
        sprite_path: "horn-beast.png",
        hp: 46,
        attack: 8,
        defense: 5,
        weakness: None,
        exp: 5,
    },
    // level 3
    Enemy {
        name: "Cobra",
        sprite_path: "cobra.png",
        hp: 51,
        attack: 9,
        defense: 6,
        weakness: None,
        exp: 5,
    },
];

const ENEMIES_WILDERNESS2: &[Enemy] = &[
    // level 7
    Enemy {
        name: "Dragonfly",
        sprite_path: "dragonfly.png",
        hp: 88,
        attack: 19,
        defense: 16,
        weakness: Some(Magic::FireEdge),
        exp: 5,
    },
    // level 8
    Enemy {
        name: "Leech",
        sprite_path: "leech.png",
        hp: 98,
        attack: 22,
        defense: 18,
        weakness: None,
        exp: 5,
    },
    // level 9
    Enemy {
        name: "Shambler",
        sprite_path: "shambler.png",
        hp: 107,
        attack: 24,
        defense: 21,
        weakness: None,
        exp: 5,
    },
    // level 9.5
    Enemy {
        name: "Fang Frog",
        sprite_path: "fang-frog.png",
        hp: 112,
        attack: 25,
        defense: 22,
        weakness: None,
        exp: 5,
    },
];

const ENEMIES_WILDERNESS3: &[Enemy] = &[Enemy {
    name: "Rat Lv16",
    sprite_path: "rat.png",
    hp: 172,
    attack: 41,
    defense: 38,
    weakness: Some(Magic::FireEdge),
    exp: 5,
}];

const ENEMIES_EARTH_CASTLE: &[Enemy] = &[
    // level 3.5
    Enemy {
        name: "Bat",
        sprite_path: "bat.png",
        hp: 56,
        attack: 10,
        defense: 7,
        weakness: None,
        exp: 5,
    },
    // level 4
    Enemy {
        name: "Scorpion",
        sprite_path: "scorpion.png",
        hp: 60,
        attack: 12,
        defense: 8,
        weakness: None,
        exp: 5,
    },
    // level 5
    Enemy {
        name: "Rogue",
        sprite_path: "rogue.png",
        hp: 70,
        attack: 14,
        defense: 11,
        weakness: None,
        exp: 5,
    },
    // level 6
    Enemy {
        name: "Golem",
        sprite_path: "golem.png",
        hp: 79,
        attack: 16,
        defense: 14,
        weakness: Some(Magic::FireEdge),
        exp: 5,
    },
];

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
