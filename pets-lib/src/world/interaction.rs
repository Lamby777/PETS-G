//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::prelude::*;

use godot::engine::{Area2D, Area2DVirtual};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct InteractionZone {
    #[base]
    node: Base<Area2D>,
}

#[godot_api]
impl Area2DVirtual for InteractionZone {
    fn init(node: Base<Area2D>) -> Self {
        Self { node }
    }

    // fn ready(&mut self) {}
}
