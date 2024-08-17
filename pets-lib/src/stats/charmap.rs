use godot::classes::file_access::ModeFlags;
use godot::prelude::*;
use io::Read as _;

use crate::prelude::*;

const CHARMAP_PATH: &str = "res://assets/charmap.json";

#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct CharMap(pub Vec<Rc<RefCell<CharData>>>);

impl CharMap {
    pub fn new() -> Self {
        CharMap(Vec::new())
    }

    pub fn character(&self, ch: &PChar) -> Rc<RefCell<CharData>> {
        self.0
            .iter()
            .find(|cd| cd.borrow().id == *ch)
            .expect("Character not found")
            .clone()
    }
}

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> CharMap {
    let mut file =
        GFile::open(CHARMAP_PATH, ModeFlags::READ).expect("charmap not found");

    let mut content = vec![];
    file.read_to_end(&mut content).unwrap();
    let content = String::from_utf8(content).unwrap();
    serde_json::from_str(&content).expect("deserialization of charmap failed")
}
