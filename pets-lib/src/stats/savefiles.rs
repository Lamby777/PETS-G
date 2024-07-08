//!
//! This file is for saving/loading the game.
//!

use io::Write;

use godot::engine::file_access::ModeFlags;
use godot::prelude::*;

use super::charmap::default_charmap;
use super::scrapbook::Scrapbook;
use crate::prelude::*;

fn save_path(slot: u8) -> String {
    format!("user://save{}.json", slot)
}

/// All the data saved to one of the save file slots
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub chars: CharMap,

    pub inventory: Rc<RefCell<Vec<Item>>>,
    pub scrapbook: Scrapbook,
    pub quests: HashMap<String, QuestPhase>,

    pub bed_color: String,
}

impl SaveFile {
    pub fn fresh() -> Self {
        let chars = default_charmap();

        Self {
            chars,
            inventory: Rc::new(RefCell::new(vec![])),
            scrapbook: Scrapbook::empty(),
            quests: HashMap::new(),
            bed_color: "red".to_string(),
        }
    }

    pub fn load_from(save_slot: u8) -> io::Result<Self> {
        let new_save = Self::fresh();
        new_save.write_to(save_slot);

        return Ok(new_save);
        // SKIP THIS SHIT FOR DEBUG PURPOSES

        // let path = save_path(save_slot);
        // let Ok(mut file) = GFile::open(path, ModeFlags::READ) else {
        //     let new_save = Self::fresh();
        //     new_save.write_to(save_slot);
        //     return Ok(new_save);
        // };
        //
        // let mut content = vec![];
        // file.read_to_end(&mut content);
        // let content = String::from_utf8(content).unwrap();
        //
        // // TODO load save file
        // // todo!()
        // Ok(serde_json::from_str(&content).unwrap())
    }

    pub fn write_to(&self, save_slot: u8) {
        let path = save_path(save_slot);
        let mut file = GFile::open(path, ModeFlags::WRITE).unwrap();

        let content = serde_json::to_string(self).unwrap();
        write!(file, "{}", content).unwrap();
    }
}
