#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Magic {
    Heal,
    EarthEdge,
    WaterEdge,
    FireEdge,
}

pub struct MagicSlot {
    pub magic: Magic,
    pub known: bool,
}

pub struct Progress {
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub level: i32,
    pub exp: i32,
    pub magic: Vec<MagicSlot>,
}

impl Magic {
    pub fn name(self) -> &'static str {
        match self {
            Self::Heal => "Heal",
            Self::EarthEdge => "EarthEdge",
            Self::WaterEdge => "WaterEdge",
            Self::FireEdge => "FireEdge",
        }
    }

    pub fn mp_cost(self) -> i32 {
        match self {
            Self::Heal => 5,
            Self::EarthEdge => 2,
            Self::WaterEdge => 2,
            Self::FireEdge => 2,
        }
    }
}

impl MagicSlot {
    pub fn battle_menu_entry(&self) -> String {
        if self.known {
            format!("{:9.9}{:2}MP", self.magic.name(), self.magic.mp_cost())
        } else {
            String::new()
        }
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            hp: 50,
            max_hp: 50,
            mp: 15,
            max_mp: 15,
            attack: 8,
            defense: 5,
            level: 1,
            exp: 0,
            magic: vec![
                MagicSlot {
                    magic: Magic::Heal,
                    known: true,
                },
                MagicSlot {
                    magic: Magic::EarthEdge,
                    known: true,
                },
                MagicSlot {
                    magic: Magic::WaterEdge,
                    known: true,
                },
                MagicSlot {
                    magic: Magic::FireEdge,
                    known: true,
                },
            ],
        }
    }
}
