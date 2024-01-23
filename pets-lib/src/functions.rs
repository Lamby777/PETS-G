//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use crate::prelude::*;

/// Autoload for functions that may need to be
/// called from anywhere in the game's code.
#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct FnInterface {}

#[godot_api]
impl FnInterface {
    /// Get a shared ref to the singleton to store in other node structs
    pub fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton("Functions".into())
            .unwrap()
            .cast()
    }
}
