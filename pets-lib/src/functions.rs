//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::prelude::*;

use crate::prelude::*;

/// Autoload for functions that may need to be
/// called from anywhere in the game's code.
#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct FnInterface {}

impl Autoload for FnInterface {
    const AUTOLOAD_NAME: &'static str = "Functions";
}

#[godot_api]
impl FnInterface {}
