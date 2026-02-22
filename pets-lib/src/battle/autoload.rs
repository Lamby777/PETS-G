//!
//! Singleton for battle-related data
//!

use godot::prelude::*;

use crate::common::*;

pub struct Battlefield {
    /// The enemies that are currently in battle with you
    pub enemies: Vec<EnemyData>,
}

impl Battlefield {
    /// Constructor to make an empty battlefield
    fn empty() -> Self {
        Self { enemies: vec![] }
    }

    /// Reset the battlefield without granting any rewards
    pub fn flee(&mut self) {
        // self.enemies.clear();
        *self = Self::empty();
    }
}

#[derive(GodotClass)]
#[class(base=Object)]
pub struct BattleInterface {
    base: Base<Object>,

    /// Battle-related info. `None` if not in battle.
    pub battlefield: Option<Battlefield>,
}

#[godot_api]
impl BattleInterface {
    #[func]
    pub fn gyat(&self) -> i32 {
        67
    }
}

impl GodotAutoload for BattleInterface {
    const AUTOLOAD_NAME: &str = "BattleInterface";
}

#[godot_api]
impl IObject for BattleInterface {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            battlefield: Battlefield::empty(),
        }
    }
}
