//!
//! This file is for saving/loading the game.
//!

use super::CharMap;
use godot::engine::{file_access::ModeFlags, FileAccess};
use serde::{Deserialize, Serialize};

/// All the data saved to one of the save file slots
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub chars: CharMap,
}

impl SaveFile {
    pub fn new_empty() -> Self {
        Self {
            chars: CharMap::new(),
        }
    }

    pub fn load_from(save_slot: u8) -> Option<Self> {
        // TODO load with serde

        let path = format!("user://save{}.json", save_slot);
        let file = FileAccess::open(path.into(), ModeFlags::READ)?;
        let _content = file.get_as_text();

        todo!()
    }
}
