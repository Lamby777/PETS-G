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
    /// shorthand for `callv` with no args
    pub fn call(name: &str) {
        Self::callv(name, VariantArray::new())
    }

    /// Call a global function by name with args
    pub fn callv(name: &str, args: VariantArray) {
        let callable = Self::singleton().callable(name);
        callable.callv(args);
    }

    #[func]
    pub fn debug_battle() {
        // TODO rename `battle_engine` to `battle`
        change_scene!("battle_engine");
    }
}
