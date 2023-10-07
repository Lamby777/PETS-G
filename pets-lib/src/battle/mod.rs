//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::global::Key;
use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

mod player;
mod rhythm;
mod stat_translation;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for BattleEngine {
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
