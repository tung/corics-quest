use miniserde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ArmorV1 {
    pub name: String,
    pub defense: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum ItemV1 {
    Salve,
    XSalve,
    Tonic,
    XTonic,
}

#[derive(Deserialize, Serialize)]
pub struct ItemSlotV1 {
    pub item: ItemV1,
    pub amount: i32,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum MagicV1 {
    Heal,
    EarthEdge,
    WaterEdge,
    FireEdge,
}

#[derive(Deserialize, Serialize)]
pub struct MagicSlotV1 {
    pub magic: MagicV1,
    pub known: bool,
}

#[derive(Deserialize, Serialize)]
pub struct WeaponV1 {
    pub name: String,
    pub attack: i32,
}

#[derive(Deserialize, Serialize)]
pub struct ProgressV1 {
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub level: i32,
    pub exp: i32,
    pub base_exp: i32,
    pub weapon: Option<WeaponV1>,
    pub armor: Option<ArmorV1>,
    pub items: Vec<ItemSlotV1>,
    pub magic: Vec<MagicSlotV1>,
    pub collected_chests: Vec<String>,
    pub turned_levers: Vec<String>,
    pub steps: Vec<i32>,
    pub earth_defeated: bool,
    pub water_defeated: bool,
    pub fire_defeated: bool,
}

impl ProgressV1 {
    pub fn from_str(data: &str, version: u32) -> Result<Self, &'static str> {
        #[allow(clippy::comparison_chain)]
        if version < 1 {
            Err("invalid save version")
        } else if version == 1 {
            miniserde::json::from_str(data).map_err(|_| "failed to parse save data")
        } else {
            Err("unknown save version")
        }
    }
}

impl std::fmt::Display for ProgressV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&miniserde::json::to_string(self))
    }
}
