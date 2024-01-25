use miniserde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpriteSheet {
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub filename: String,
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Rect,
    #[serde(rename = "sourceSize")]
    pub source_size: Size,
    pub duration: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Size,
    pub scale: String,
    #[serde(rename = "frameTags")]
    pub frame_tags: Vec<FrameTag>,
}

#[derive(Serialize, Deserialize)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub direction: String,
    pub color: String,
}
