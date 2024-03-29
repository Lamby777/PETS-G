use crate::prelude::*;
use godot::prelude::*;

/// Autoload for functions that may need to be
/// called from anywhere in the game's code.
#[derive(GodotClass)]
#[class(init, base=Object)]
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

    // #[func]
    // pub fn debug_llm() {
    //     crate::llm::llm_generate();
    // }

    #[func]
    pub fn debug_battle() {
        let dbg_eid = EnemyID::A_NONNY_MOUSE;
        World::start_battle(dbg_eid.into());
    }
}
