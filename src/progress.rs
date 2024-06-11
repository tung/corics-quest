pub struct Armor {
    pub name: String,
    pub defense: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Item {
    Salve,
    XSalve,
    Tonic,
    XTonic,
}

pub struct ItemSlot {
    pub item: Item,
    pub amount: i32,
}

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

pub struct Weapon {
    pub name: String,
    pub attack: i32,
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
    pub base_exp: i32,
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    pub items: Vec<ItemSlot>,
    pub magic: Vec<MagicSlot>,
    pub collected_chests: Vec<String>,
    pub turned_levers: Vec<String>,
    pub earth_defeated: bool,
    pub water_defeated: bool,
    pub fire_defeated: bool,
}

#[rustfmt::skip]
const EXP_FOR_NEXT_LEVEL: [i32; 29] = [
    40, 80, 100, 120, 275, 450, 480, 500, 825, 1160,
    1200, 1230, 1940, 2680, 2735, 2785, 4350, 5975, 6065, 6150,
    9575, 13080, 13235, 13380, 16000, 19250, 23000, 28000, 33300,
];

impl Item {
    fn description(self) -> &'static str {
        match self {
            Self::Salve => "Heals 30% of max HP.",
            Self::XSalve => "Heals all HP.",
            Self::Tonic => "Restores 30% of max MP.",
            Self::XTonic => "Restores all MP.",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Salve => "Salve",
            Self::XSalve => "XSalve",
            Self::Tonic => "Tonic",
            Self::XTonic => "XTonic",
        }
    }
}

impl ItemSlot {
    pub fn description(&self) -> &'static str {
        if self.amount > 0 {
            self.item.description()
        } else {
            ""
        }
    }

    pub fn battle_menu_entry(&self) -> String {
        if self.amount > 0 {
            format!("{:9.9}{:2}", self.item.name(), self.amount)
        } else {
            String::new()
        }
    }

    pub fn main_menu_entry(&self) -> String {
        if self.amount > 0 {
            format!("{:16.16} {:2} / 9", self.item.name(), self.amount)
        } else {
            String::new()
        }
    }
}

impl Magic {
    fn description(self) -> &'static str {
        match self {
            Self::Heal => "Heals 50% of max HP.",
            Self::EarthEdge => "Deals earth damage.",
            Self::WaterEdge => "Deals water damage.",
            Self::FireEdge => "Deals fire damage.",
        }
    }

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
            Self::Heal => 4,
            Self::EarthEdge => 1,
            Self::WaterEdge => 1,
            Self::FireEdge => 1,
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

    pub fn description(&self) -> &'static str {
        if self.known {
            self.magic.description()
        } else {
            ""
        }
    }

    pub fn main_menu_entry(&self) -> String {
        if self.known {
            format!("{:18.18}{:2} MP", self.magic.name(), self.magic.mp_cost())
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
            attack: 6,
            defense: 3,
            level: 1,
            exp: 0,
            base_exp: 0,
            weapon: None,
            armor: None,
            items: vec![
                ItemSlot {
                    item: Item::Salve,
                    amount: 4,
                },
                ItemSlot {
                    item: Item::XSalve,
                    amount: 3,
                },
                ItemSlot {
                    item: Item::Tonic,
                    amount: 2,
                },
                ItemSlot {
                    item: Item::XTonic,
                    amount: 1,
                },
            ],
            magic: vec![
                MagicSlot {
                    magic: Magic::Heal,
                    known: true,
                },
                MagicSlot {
                    magic: Magic::FireEdge,
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
            ],
            collected_chests: Vec::new(),
            turned_levers: Vec::new(),
            earth_defeated: false,
            water_defeated: false,
            fire_defeated: false,
        }
    }

    pub fn gain_level(&mut self) {
        if let Some(next_exp) = self.next_exp() {
            self.base_exp += next_exp;
        }
        self.level += 1;
        self.max_hp += 30;
        self.hp += 30;
        self.max_mp += 1;
        self.mp += 1;
        self.attack += 2;
        self.defense += 2;
    }

    pub fn gain_level_from_exp(&mut self) -> bool {
        let Some(next_exp) = self.next_exp() else {
            return false;
        };
        if self.exp < next_exp {
            return false;
        }
        self.exp -= next_exp;
        self.gain_level();
        true
    }

    pub fn maybe_give_items(&mut self, item: Item, min_amount: i32) -> i32 {
        let item_slot = match self.items.iter_mut().find(|s| s.item == item) {
            Some(s) => s,
            None => panic!("progress item slot for {:?}", item),
        };
        if item_slot.amount < min_amount {
            let given = min_amount - item_slot.amount;
            item_slot.amount = min_amount;
            given
        } else {
            0
        }
    }

    pub fn maybe_upgrade_armor(&mut self, name: &str, defense: i32) -> bool {
        let current_armor_defense = self.armor.as_ref().map(|a| a.defense).unwrap_or(0);
        if defense <= current_armor_defense {
            false
        } else {
            self.defense += defense - current_armor_defense;
            self.armor = Some(Armor {
                name: name.to_string(),
                defense,
            });
            true
        }
    }

    pub fn maybe_upgrade_weapon(&mut self, name: &str, attack: i32) -> bool {
        let current_weapon_attack = self.weapon.as_ref().map(|w| w.attack).unwrap_or(0);
        if attack <= current_weapon_attack {
            false
        } else {
            self.attack += attack - current_weapon_attack;
            self.weapon = Some(Weapon {
                name: name.to_string(),
                attack,
            });
            true
        }
    }

    pub fn next_exp(&self) -> Option<i32> {
        assert!(self.level >= 1);
        let index = usize::try_from(self.level).expect("progress.level as usize") - 1;
        EXP_FOR_NEXT_LEVEL.get(index).copied()
    }
}

pub fn player_rank(level: i32) -> &'static str {
    match level {
        ..=7 => "Fighter",
        8..=15 => "Warrior",
        16..=23 => "Knight",
        24..=29 => "Valor Guard",
        30.. => "Blademaster",
    }
}
