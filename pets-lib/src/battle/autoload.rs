//!
//! Singleton for accessing battle-related state
//!

use std::sync::LazyLock;

use godot::global::randomize;
use godot::prelude::*;

use crate::common::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct BattleInterface {
    base: Base<Object>,

    pub save: SaveFile,
}

#[godot_api]
impl BattleInterface {
    pub fn your_mom(&mut self) {
        todo!()
    }
}

impl GodotAutoload for BattleInterface {
    const AUTOLOAD_NAME: &str = "Battle";
}

#[godot_api]
impl IObject for BattleInterface {
    fn init(base: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let save = SaveFile::fresh();

        // load registries, cuz they're `LazyLock`s
        LazyLock::force(&REGISTRIES);

        // randomize seed for godot
        randomize();

        Self { base, save }
    }
}
