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
    fn _empty() -> Self {
        Self { enemies: vec![] }
    }

    fn from_enemies(enemies: &[EnemyData]) -> Self {
        Self {
            enemies: enemies.to_vec(),
        }
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

    pub fn start_battle(&mut self, first_enemies: &[EnemyData]) {
        if self.battlefield.is_some() {
            panic!("called `start_battle` while one is already taking place");
        }

        self.battlefield = Some(Battlefield::from_enemies(first_enemies));
    }

    pub fn _push_enemy(&mut self, enemy: &EnemyData) {
        self.battlefield
            .as_mut()
            .unwrap()
            .enemies
            .push(enemy.clone());
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
