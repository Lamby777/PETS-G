//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::prelude::*;

use godot::engine::{Area2D, Area2DVirtual, Engine};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct InteractionManager {
    #[base]
    node: Base<Area2D>,
}

#[godot_api]
impl InteractionManager {
    #[signal]
    fn register_zone() {
        //
    }

    /// Get a shared ref to the singleton to store in other node structs
    pub fn singleton() -> Gd<InteractionManager> {
        Engine::singleton()
            .get_singleton("Interactions".into())
            .unwrap()
            .cast()
    }
}

#[godot_api]
impl Area2DVirtual for InteractionManager {
    fn init(node: Base<Area2D>) -> Self {
        Self { node }
    }
}
