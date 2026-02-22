//!
//! Singleton for battle-related data
//!

use std::borrow::Borrow;

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
    pub fn is_in_battle(&self) -> bool {
        self.battlefield.is_some()
    }

    pub fn push_enemy(&mut self, enemy: impl Borrow<EnemyData>) {
        let Some(ref mut battlefield) = self.battlefield else {
            panic!("bruh");
        };

        battlefield.enemies.push(enemy.borrow().clone());
    }

    /// Reset the battlefield without granting any rewards
    pub fn flee(&mut self) {
        self.battlefield = None;
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
            battlefield: None,
        }
    }
}
