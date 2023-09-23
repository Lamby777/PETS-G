//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::{Engine, Node2D, Node2DVirtual};
use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::savefiles::SaveFile;
use super::CharData;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct StatsInterface {
    #[base]
    node: Base<Node2D>,

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
    pub fn get_character(&self, ch: String) -> Rc<RefCell<CharData>> {
        self.save
            .chars
            .get(&ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

#[godot_api]
impl Node2DVirtual for StatsInterface {
    fn init(node: Base<Node2D>) -> Self {
        // start empty, load other if the player
        // picks a save file instead of "new"
        let charmap = SaveFile::new_empty();

        Self {
            node,
            save: charmap,
        }
    }
}
