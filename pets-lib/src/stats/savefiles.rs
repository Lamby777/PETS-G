//!
//! This file is for saving/loading the game.
//!

use crate::prelude::*;

use godot::engine::file_access::ModeFlags;
use godot::engine::FileAccess;

use super::charmap::default_charmap;

/// All the data saved to one of the save file slots
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub chars: CharMap,
    pub inventory: Vec<Item>,
}

#[allow(unused)]
impl SaveFile {
    pub fn fresh() -> Self {
        let (chars, _) = default_charmap();

        Self {
            chars,
            inventory: vec![],
        }
    }

    pub fn load_from(save_slot: u8) -> Option<Self> {
        // TODO load save file
        let path = format!("user://save{}.json", save_slot);
        let file = FileAccess::open(path.into(), ModeFlags::READ)?;
        let _content = file.get_as_text();

        todo!()
    }
}
