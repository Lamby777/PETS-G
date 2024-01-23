//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use crate::prelude::*;

/// Autoload class for easy management of dialog boxes
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
