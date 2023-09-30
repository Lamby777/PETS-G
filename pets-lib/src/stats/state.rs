//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::prelude::*;

use super::charmap::default_charmap;
use super::statcalc::CharStatCalcs;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    #[base]
    node: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    save: SaveFile,

    /// Hash map of function pointers for calculating stats
    statcalcs: HashMap<String, CharStatCalcs>,
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

    pub fn get_statcalc(&self, ch: &str) -> CharStatCalcs {
        self.statcalcs
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

#[godot_api]
impl ObjectVirtual for StatsInterface {
    fn init(node: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let charmap = default_charmap();
        let save = SaveFile { chars: charmap };

        Self {
            node,
            save,
            statcalcs: todo!(),
        }
    }
}
