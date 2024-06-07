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
    pub actions: &'static [EnemyAction],
}

#[derive(Clone)]
pub struct EnemyAction {
    pub chance: u32,
    pub msg: &'static str,
    pub damage_factor: Option<f32>,
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
        exp: 10,
        actions: &[EnemyAction {
            chance: 20,
            msg: "twitches its whiskers.",
            damage_factor: None,
        }],
    },
    // level 2
    Enemy {
        name: "Dog",
        sprite_path: "dog.png",
        hp: 42,
        attack: 6,
        defense: 3,
        weakness: None,
        exp: 15,
        actions: &[EnemyAction {
            chance: 20,
            msg: "growls at Coric.",
            damage_factor: None,
        }],
    },
    // level 3
    Enemy {
        name: "Horn Beast",
        sprite_path: "horn-beast.png",
        hp: 51,
        attack: 9,
        defense: 6,
        weakness: None,
        exp: 22,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "charges at Coric!",
                damage_factor: Some(1.3),
            },
            EnemyAction {
                chance: 15,
                msg: "charges at Coric!\nCoric steps aside!",
                damage_factor: None,
            },
        ],
    },
    // level 4
    Enemy {
        name: "Cobra",
        sprite_path: "cobra.png",
        hp: 60,
        attack: 12,
        defense: 8,
        weakness: None,
        exp: 30,
        actions: &[EnemyAction {
            chance: 20,
            msg: "hisses at Coric.",
            damage_factor: None,
        }],
    },
];

const ENEMIES_WILDERNESS2: &[Enemy] = &[
    // level 9
    Enemy {
        name: "Dragonfly",
        sprite_path: "dragonfly.png",
        hp: 107,
        attack: 24,
        defense: 21,
        weakness: Some(Magic::FireEdge),
        exp: 96,
        actions: &[
            EnemyAction {
                chance: 10,
                msg: "swoops forward and bites!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 20,
                msg: "hovers to and fro.",
                damage_factor: None,
            },
        ],
    },
    // level 10
    Enemy {
        name: "Leech",
        sprite_path: "leech.png",
        hp: 116,
        attack: 27,
        defense: 23,
        weakness: None,
        exp: 115,
        actions: &[EnemyAction {
            chance: 20,
            msg: "bites Coric!",
            damage_factor: Some(1.3),
        }],
    },
    // level 11
    Enemy {
        name: "Shambler",
        sprite_path: "shambler.png",
        hp: 126,
        attack: 29,
        defense: 26,
        weakness: None,
        exp: 139,
        actions: &[EnemyAction {
            chance: 15,
            msg: "spits a stinger at Coric!",
            damage_factor: Some(1.5),
        }],
    },
    // level 12
    Enemy {
        name: "Fang Frog",
        sprite_path: "fang-frog.png",
        hp: 135,
        attack: 32,
        defense: 28,
        weakness: None,
        exp: 166,
        actions: &[
            EnemyAction {
                chance: 20,
                msg: "lunges and bites Coric!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 10,
                msg: "lunges at Coric!\nCoric narrowly dodges!",
                damage_factor: None,
            },
        ],
    },
];

const ENEMIES_WILDERNESS3: &[Enemy] = &[
    // level 17
    Enemy {
        name: "Griffon",
        sprite_path: "griffon.png",
        hp: 182,
        attack: 44,
        defense: 41,
        weakness: Some(Magic::EarthEdge),
        exp: 414,
        actions: &[EnemyAction {
            chance: 30,
            msg: "swoops with its claws bared!",
            damage_factor: Some(1.3),
        }],
    },
    // level 18
    Enemy {
        name: "Orc",
        sprite_path: "orc.png",
        hp: 191,
        attack: 47,
        defense: 43,
        weakness: None,
        exp: 496,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "stabs Coric with its dagger!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 15,
                msg: "thrusts its dagger!\nCoric deflects the attack!",
                damage_factor: None,
            },
        ],
    },
    // level 19
    Enemy {
        name: "Troll",
        sprite_path: "troll.png",
        hp: 200,
        attack: 49,
        defense: 46,
        weakness: Some(Magic::WaterEdge),
        exp: 596,
        actions: &[
            EnemyAction {
                chance: 20,
                msg: "swipes Coric with its claws!",
                damage_factor: Some(1.3),
            },
            EnemyAction {
                chance: 20,
                msg: "swings its claws wildly!\nCoric blocks some strikes.",
                damage_factor: Some(0.5),
            },
        ],
    },
    // level 20
    Enemy {
        name: "War Tusk",
        sprite_path: "war-tusk.png",
        hp: 210,
        attack: 51,
        defense: 49,
        weakness: None,
        exp: 715,
        actions: &[
            EnemyAction {
                chance: 10,
                msg: "charges and gouges Coric!",
                damage_factor: Some(1.8),
            },
            EnemyAction {
                chance: 20,
                msg: "charges at Coric!\nCoric leaps aside!",
                damage_factor: None,
            },
        ],
    },
];

const ENEMIES_EARTH_CASTLE: &[Enemy] = &[
    // level 5
    Enemy {
        name: "Bat",
        sprite_path: "bat.png",
        hp: 70,
        attack: 14,
        defense: 11,
        weakness: None,
        exp: 41,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "bites Coric!",
                damage_factor: Some(1.3),
            },
            EnemyAction {
                chance: 15,
                msg: "screeches and flutters about.",
                damage_factor: None,
            },
        ],
    },
    // level 6
    Enemy {
        name: "Scorpion",
        sprite_path: "scorpion.png",
        hp: 79,
        attack: 16,
        defense: 14,
        weakness: None,
        exp: 53,
        actions: &[EnemyAction {
            chance: 20,
            msg: "thrusts its stinger!",
            damage_factor: Some(1.4),
        }],
    },
    // level 7
    Enemy {
        name: "Rogue",
        sprite_path: "rogue.png",
        hp: 88,
        attack: 19,
        defense: 16,
        weakness: None,
        exp: 67,
        actions: &[
            EnemyAction {
                chance: 20,
                msg: "throws a knife at Coric!",
                damage_factor: Some(1.3),
            },
            EnemyAction {
                chance: 10,
                msg: "tosses a knife!\nIt barely grazes Coric!",
                damage_factor: Some(0.3),
            },
        ],
    },
    // level 8
    Enemy {
        name: "Golem",
        sprite_path: "golem.png",
        hp: 98,
        attack: 22,
        defense: 19,
        weakness: Some(Magic::FireEdge),
        exp: 80,
        actions: &[EnemyAction {
            chance: 15,
            msg: "swings its stony fist!",
            damage_factor: Some(1.4),
        }],
    },
];

const ENEMIES_WATER_CASTLE: &[Enemy] = &[
    // level 13
    Enemy {
        name: "Jelly",
        sprite_path: "jelly.png",
        hp: 144,
        attack: 34,
        defense: 31,
        weakness: None,
        exp: 199,
        actions: &[EnemyAction {
            chance: 10,
            msg: "quivers in place.",
            damage_factor: None,
        }],
    },
    // level 14
    Enemy {
        name: "Ghost",
        sprite_path: "ghost.png",
        hp: 154,
        attack: 37,
        defense: 33,
        weakness: None,
        exp: 239,
        actions: &[
            EnemyAction {
                chance: 20,
                msg: "extends its ethereal touch!",
                damage_factor: Some(1.3),
            },
            EnemyAction {
                chance: 20,
                msg: "emits a chilling breeze!",
                damage_factor: Some(0.7),
            },
        ],
    },
    // level 15
    Enemy {
        name: "Turtle",
        sprite_path: "turtle.png",
        hp: 163,
        attack: 39,
        defense: 66,
        weakness: None,
        exp: 287,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "extends its neck and bites!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 10,
                msg: "slowly advances on Coric.",
                damage_factor: None,
            },
        ],
    },
    // level 16
    Enemy {
        name: "Serpent",
        sprite_path: "serpent.png",
        hp: 172,
        attack: 42,
        defense: 39,
        weakness: Some(Magic::EarthEdge),
        exp: 345,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "fires a jet of water!",
                damage_factor: Some(1.7),
            },
            EnemyAction {
                chance: 15,
                msg: "fires a jet of water!\nCoric dodges some of it!",
                damage_factor: Some(0.5),
            },
            EnemyAction {
                chance: 15,
                msg: "roars in anger!",
                damage_factor: None,
            },
        ],
    },
];

const ENEMIES_FIRE_CASTLE: &[Enemy] = &[
    // level 21
    Enemy {
        name: "Basilisk",
        sprite_path: "basilisk.png",
        hp: 219,
        attack: 54,
        defense: 51,
        weakness: None,
        exp: 858,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "casts its burning gaze!",
                damage_factor: Some(1.4),
            },
            EnemyAction {
                chance: 15,
                msg: "casts its burning gaze!\nCoric narrowly averts his eyes!",
                damage_factor: None,
            },
        ],
    },
    // level 22
    Enemy {
        name: "Warlock",
        sprite_path: "warlock.png",
        hp: 228,
        attack: 57,
        defense: 53,
        weakness: None,
        exp: 1029,
        actions: &[
            EnemyAction {
                chance: 20,
                msg: "conjures infernal bolts!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 10,
                msg: "mutters incoherent curses.",
                damage_factor: None,
            },
        ],
    },
    // level 23
    Enemy {
        name: "Minotaur",
        sprite_path: "minotaur.png",
        hp: 238,
        attack: 59,
        defense: 56,
        weakness: None,
        exp: 1235,
        actions: &[
            EnemyAction {
                chance: 15,
                msg: "swings its huge axe!",
                damage_factor: Some(1.5),
            },
            EnemyAction {
                chance: 15,
                msg: "swings its huge axe!\nCoric blocks the strike!",
                damage_factor: None,
            },
            EnemyAction {
                chance: 10,
                msg: "grunts in anger.",
                damage_factor: None,
            },
        ],
    },
    // level 24
    Enemy {
        name: "Vampire",
        sprite_path: "vampire.png",
        hp: 247,
        attack: 62,
        defense: 59,
        weakness: Some(Magic::WaterEdge),
        exp: 1482,
        actions: &[
            EnemyAction {
                chance: 25,
                msg: "sinks its fangs into Coric!",
                damage_factor: Some(1.7),
            },
            EnemyAction {
                chance: 10,
                msg: "lunges at Coric!\nCoric barely dodges!",
                damage_factor: None,
            },
            EnemyAction {
                chance: 10,
                msg: "bares its fangs.",
                damage_factor: None,
            },
        ],
    },
];

impl EncounterGroup {
    pub fn random_enemy(self, rng: &mut Rng) -> Enemy {
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
        group[rng.random(i) as usize].clone()
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
