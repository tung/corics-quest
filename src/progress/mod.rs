// # Save Compatibility
//
// Progress is not saved or loaded directly; instead, in-game progress structs and enums are
// converted to storage-stable forms, which in turn are saved to and loaded from persistent
// storage.
//
// Storage-stable structs and enums are *versioned*; these exist as submodules, e.g. v1, v2, v3
// and so on.  Each submodule deals only with its own versioned data, the data of the previous
// version (if any), and strings, typically by serializing to or deserializing from JSON.
//
// Code in this file *only* directly interacts with the submodule of the latest version; the other
// versions are kept for backwards compatibility with older save data.
//
// When saving (relevant code location in square brackets):
//
// 1. [here] In-game progress structs and enums are converted to the most recent storage-stable
//    version.
// 2. [version submodule] The storage-stable data is converted into a string (JSON).
// 3. [here] The string is written to stable storage, along with a header and version number.
//
// When loading:
//
// 1. [here] The save data header is split into a version number and unparsed data.
// 2. [version submodule] Loading code checks the version number before dealing with the data.
// 3. [version submodule] If the version matches, parse the data and return it.
// 4. [version submodule] If the version is older, send it to the loading code in the submodule for
//    the previous version, then adapt it to the current version as needed.
// 5. [here] Convert the storage-stable data back to in-game progress structs and enums.
//
// Versions should not be changed once exposed to players.  If the save format needs to be changed,
// e.g. v1 exists, but you need a v2:
//
// 1.  [here] Make any changes needed to the in-game progress structs and enums.  You may need to
//     stub out some code in the `From` implementations below for testing purposes.
// 2.  [here] Add `mod v2` for a new version submodule.
// 3.  [here] Replace `use v1::*` with `use v2::*`.
// 4.  [here] Change `SAVE_VERSION` to `2`.
// 5.  [src/progress/] Create a new submodule file: `src/progress/v2.rs`.
// 6.  [v2.rs] Add `use super::v1::*;`.
// 7.  [v2.rs] Add `use miniserde::{Deserialize, Serialize};`.
// 8.  [v2.rs] Copy-and-paste the current in-game progress structs and enums that have changed,
//     suffix them with "V2" and add `Serialize` and `Deserialize` derives to them.  Reuse "V1"
//     structs and enums for anything that hasn't changed.
// 9.  [here] Implement `From<&Progress>` for `ProgressV2` and friends so saving works.
// 10. [v2.rs] Write a `std::fmt::Display` implementation for `ProgressV2` that serializes itself
//     into string form for saving.
// 11. [here] Implement `From<ProgressV2>` for `Progress` and friends so loading works for save
//     data in the most recent version.
// 12. [v2.rs] Implement `From<ProgressV1>` for `ProgressV2` and friends so loading works for older
//     versions too.
// 13. [v2.rs] Write a `ProgressV2::from_str` function that loads itself from a string slice or
//     calls `ProgressV1::from_str` and adapts the result into a `ProgressV2`, depending on the
//     version number.
// 14. [v1.rs, the *previous* version sub-module] Clean up unused code, namely the `Serialize`
//     derives and the old `std::fmt::Display` implementation for `ProgressV1`; we never save data
//     with older versions once a new one is available.
//
// Keep in mind that changes to progress data might not always need a new version, e.g.
//
// - Removed data can just save dummy data that's ignored on load.
// - Minor data changes can just be adapted in the `From` trait implementations in this file.
// - In-development versions can just change the most recent storage-stable format directly and
//   deal with internal save format breakage manually.

mod v1;

use v1::*;

use crate::enemy::*;

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
    pub steps: Vec<i32>,
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

const SAVE_KEY: &str = "save";
const SAVE_HEADER_START: &str = "// cqsave ";
const SAVE_VERSION: u32 = 1;

impl From<&Armor> for ArmorV1 {
    fn from(s: &Armor) -> Self {
        Self {
            name: s.name.clone(),
            defense: s.defense,
        }
    }
}

impl From<ArmorV1> for Armor {
    fn from(l: ArmorV1) -> Self {
        Self {
            name: l.name,
            defense: l.defense,
        }
    }
}

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

impl From<Item> for ItemV1 {
    fn from(s: Item) -> Self {
        match s {
            Item::Salve => Self::Salve,
            Item::XSalve => Self::XSalve,
            Item::Tonic => Self::Tonic,
            Item::XTonic => Self::XTonic,
        }
    }
}

impl From<ItemV1> for Item {
    fn from(l: ItemV1) -> Self {
        match l {
            ItemV1::Salve => Self::Salve,
            ItemV1::XSalve => Self::XSalve,
            ItemV1::Tonic => Self::Tonic,
            ItemV1::XTonic => Self::XTonic,
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

impl From<&ItemSlot> for ItemSlotV1 {
    fn from(s: &ItemSlot) -> Self {
        Self {
            item: s.item.into(),
            amount: s.amount,
        }
    }
}

impl From<ItemSlotV1> for ItemSlot {
    fn from(l: ItemSlotV1) -> Self {
        Self {
            item: l.item.into(),
            amount: l.amount,
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

impl From<Magic> for MagicV1 {
    fn from(s: Magic) -> Self {
        match s {
            Magic::Heal => Self::Heal,
            Magic::EarthEdge => Self::EarthEdge,
            Magic::WaterEdge => Self::WaterEdge,
            Magic::FireEdge => Self::FireEdge,
        }
    }
}

impl From<MagicV1> for Magic {
    fn from(l: MagicV1) -> Self {
        match l {
            MagicV1::Heal => Self::Heal,
            MagicV1::EarthEdge => Self::EarthEdge,
            MagicV1::WaterEdge => Self::WaterEdge,
            MagicV1::FireEdge => Self::FireEdge,
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

impl From<&MagicSlot> for MagicSlotV1 {
    fn from(s: &MagicSlot) -> Self {
        Self {
            magic: s.magic.into(),
            known: s.known,
        }
    }
}

impl From<MagicSlotV1> for MagicSlot {
    fn from(l: MagicSlotV1) -> Self {
        Self {
            magic: l.magic.into(),
            known: l.known,
        }
    }
}

impl From<&Weapon> for WeaponV1 {
    fn from(s: &Weapon) -> Self {
        Self {
            name: s.name.clone(),
            attack: s.attack,
        }
    }
}

impl From<WeaponV1> for Weapon {
    fn from(l: WeaponV1) -> Self {
        Self {
            name: l.name,
            attack: l.attack,
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
                    amount: 0,
                },
                ItemSlot {
                    item: Item::XSalve,
                    amount: 0,
                },
                ItemSlot {
                    item: Item::Tonic,
                    amount: 0,
                },
                ItemSlot {
                    item: Item::XTonic,
                    amount: 0,
                },
            ],
            magic: vec![
                MagicSlot {
                    magic: Magic::Heal,
                    known: false,
                },
                MagicSlot {
                    magic: Magic::FireEdge,
                    known: false,
                },
                MagicSlot {
                    magic: Magic::EarthEdge,
                    known: false,
                },
                MagicSlot {
                    magic: Magic::WaterEdge,
                    known: false,
                },
            ],
            collected_chests: Vec::new(),
            turned_levers: Vec::new(),
            steps: vec![0; EncounterGroup::NUM_GROUPS + 1],
            earth_defeated: false,
            water_defeated: false,
            fire_defeated: false,
        }
    }

    pub fn load() -> Result<Self, &'static str> {
        let raw_data = quad_storage::STORAGE
            .lock()
            .map_err(|_| "storage error")?
            .get(SAVE_KEY)
            .ok_or("no save file found")?;
        let version_and_data = raw_data
            .strip_prefix(SAVE_HEADER_START)
            .ok_or("invalid save data")?;
        let (version_str, data) = version_and_data
            .split_once(|c: char| !c.is_ascii_digit())
            .ok_or("missing version number")?;
        let version = version_str
            .parse::<u32>()
            .map_err(|_| "failed to parse version")?;
        Ok(Progress::from(ProgressV1::from_str(
            data.trim_start(),
            version,
        )?))
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

    pub fn save(&self) -> Result<(), &'static str> {
        let raw_data = format!(
            "{SAVE_HEADER_START}{SAVE_VERSION}\n{}",
            &ProgressV1::from(self).to_string()
        );
        quad_storage::STORAGE
            .lock()
            .map_err(|_| "storage error")?
            .set(SAVE_KEY, &raw_data);
        Ok(())
    }
}

impl From<&Progress> for ProgressV1 {
    fn from(s: &Progress) -> Self {
        Self {
            hp: s.hp,
            max_hp: s.max_hp,
            mp: s.mp,
            max_mp: s.max_mp,
            attack: s.attack,
            defense: s.defense,
            level: s.level,
            exp: s.exp,
            base_exp: s.base_exp,
            weapon: s.weapon.as_ref().map(WeaponV1::from),
            armor: s.armor.as_ref().map(ArmorV1::from),
            items: s.items.iter().map(ItemSlotV1::from).collect(),
            magic: s.magic.iter().map(MagicSlotV1::from).collect(),
            collected_chests: s.collected_chests.clone(),
            turned_levers: s.turned_levers.clone(),
            steps: s.steps.clone(),
            earth_defeated: s.earth_defeated,
            water_defeated: s.water_defeated,
            fire_defeated: s.fire_defeated,
        }
    }
}

impl From<ProgressV1> for Progress {
    fn from(mut l: ProgressV1) -> Self {
        Self {
            hp: l.hp,
            max_hp: l.max_hp,
            mp: l.mp,
            max_mp: l.max_mp,
            attack: l.attack,
            defense: l.defense,
            level: l.level,
            exp: l.exp,
            base_exp: l.base_exp,
            weapon: l.weapon.map(Weapon::from),
            armor: l.armor.map(Armor::from),
            items: l.items.drain(..).map(ItemSlot::from).collect(),
            magic: l.magic.drain(..).map(MagicSlot::from).collect(),
            collected_chests: l.collected_chests,
            turned_levers: l.turned_levers,
            steps: l.steps,
            earth_defeated: l.earth_defeated,
            water_defeated: l.water_defeated,
            fire_defeated: l.fire_defeated,
        }
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

pub fn save_data_exists() -> bool {
    quad_storage::STORAGE
        .lock()
        .map_or(false, |s| s.get(SAVE_KEY).is_some())
}
