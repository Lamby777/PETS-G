//!
//! This file is for saving/loading the game.
//!

use super::CharMap;
use godot::engine::{file_access::ModeFlags, FileAccess};
use serde::{Deserialize, Serialize};

/// All the data saved to one of the save file slots
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    chars: CharMap,
}

impl SaveFile {
    fn load_from(save_slot: u8) -> Option<Self> {
        let path = format!("user://save{}.json", save_slot);
        let file = FileAccess::open(path.into(), ModeFlags::READ)?;
        let content = file.get_as_text();

        todo!()
    }
}
