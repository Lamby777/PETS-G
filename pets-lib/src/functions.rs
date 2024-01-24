use crate::prelude::*;
use godot::prelude::*;

/// Autoload for functions that may need to be
/// called from anywhere in the game's code.
#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct FnInterface;

impl Autoload for FnInterface {
    const AUTOLOAD_NAME: &'static str = "Functions";
}

// Because I'm totally not gonna regret this later
#[godot_api]
impl FnInterface {
    pub fn call(name: &str) {
        let callable = Self::singleton().callable(name);
        callable.callv(Array::new());
    }

    #[func]
    pub fn debug_battle() {
        godot_print!("hey :D");
    }
}
