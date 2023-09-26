//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    #[base]
    node: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    save: SaveFile,
}

#[godot_api]
impl StatsInterface {
    /// Get a shared ref to the singleton to store in other node structs
    pub fn singleton() -> Gd<StatsInterface> {
        Engine::singleton()
            .get_singleton("Stats".into())
            .unwrap()
            .cast()
    }

    // #[func]
    pub fn get_character(&self, ch: &str) -> Rc<RefCell<CharData>> {
        self.save
            .chars
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

#[godot_api]
impl ObjectVirtual for StatsInterface {
    fn init(node: Base<Object>) -> Self {
        // start empty, load other if the player
        // picks a save file instead of "new"
        let charmap = SaveFile::new_default();

        Self {
            node,
            save: charmap,
        }
    }
}
