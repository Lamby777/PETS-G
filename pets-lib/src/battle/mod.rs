//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

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
}
