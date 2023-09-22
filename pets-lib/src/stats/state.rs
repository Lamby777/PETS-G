//!
//! Singleton for accessing player stats in GDScript.
//!

use std::collections::HashMap;

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::{CharData, CharMap};

fn load_charmap(save_slot: u8) -> Option<CharMap> {
    todo!()
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct StatsInterface {
    #[base]
    node: Base<Node2D>,

    /// Hash map of info on all the different characters in the game.
    characters: CharMap,
}

#[godot_api]
impl StatsInterface {
    // #[func]
    pub fn get_character(&self, ch: String) -> Rc<RefCell<CharData>> {
        self.characters
            .get(&ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

#[godot_api]
impl Node2DVirtual for StatsInterface {
    fn init(node: Base<Node2D>) -> Self {
        let charmap = load_charmap(1).unwrap_or_else(|| todo!());

        Self {
            node,
            characters: charmap,
        }
    }
}
