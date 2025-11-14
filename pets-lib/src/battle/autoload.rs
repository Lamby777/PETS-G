//!
//! Singleton for accessing battle-related state
//!

use godot::prelude::*;

use crate::common::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct BattleInterface {
    base: Base<Object>,
}

#[godot_api]
impl BattleInterface {
    pub fn _your_mom(&mut self) {
        todo!()
    }
}

impl GodotAutoload for BattleInterface {
    const AUTOLOAD_NAME: &str = "Battle";
}

#[godot_api]
impl IObject for BattleInterface {
    fn init(base: Base<Object>) -> Self {
        Self { base }
    }
}
