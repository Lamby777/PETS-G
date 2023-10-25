//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::prelude::*;

use godot::engine::{Area2D, Area2DVirtual};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct InteractionZone {
    #[base]
    node: Base<Area2D>,

    #[export]
    name: GodotString,
}

#[godot_api]
impl InteractionZone {}

#[godot_api]
impl Area2DVirtual for InteractionZone {
    fn init(node: Base<Area2D>) -> Self {
        Self {
            node,
            name: "".into(),
        }
    }
}
