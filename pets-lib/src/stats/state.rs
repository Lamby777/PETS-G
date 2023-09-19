//!
//! Singleton for accessing player stats in GDScript.
//!

use std::collections::HashMap;

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::CharData;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct StatsInterface {
    #[base]
    node: Base<Node2D>,

    /// Hash map of info on all the different characters in the game.
    characters: HashMap<String, Rc<RefCell<CharData>>>,
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
        Self {
            node,
            characters: HashMap::new(),
        }
    }
}
