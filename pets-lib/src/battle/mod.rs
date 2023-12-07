//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

#![allow(unused)]

use godot::engine::global::Key;
use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

mod player;
mod stat_translation;

#[allow(unused)]
mod rhythm;

#[derive(Default)]
enum BattleState {
    #[default]
    /// Picking one of the options below
    Menu,

    /// Dodging attacks while clicking to the beat
    Attack,

    /// Selecting a skill to use
    Skill,

    /// Selecting an item to use
    Item,
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,

    current_state: BattleState,
}

#[godot_api]
impl INode2D for BattleEngine {
    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        // if q pressed, quit (dev shit)
        if input.is_key_pressed(Key::KEY_Q) {
            let mut tree = self.node.get_tree().unwrap();
            tree.quit();
        }
    }
}
