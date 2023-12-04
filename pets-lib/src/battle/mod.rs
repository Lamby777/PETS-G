//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::global::Key;
use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

mod player;
mod stat_translation;

#[allow(unused)]
mod rhythm;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl INode2D for BattleEngine {
    fn init(node: Base<Node2D>) -> Self {
        Self { node }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        // if q pressed, quit (dev shit)
        if input.is_key_pressed(Key::KEY_Q) {
            let mut tree = self.node.get_tree().unwrap();
            tree.quit();
        }
    }
}
