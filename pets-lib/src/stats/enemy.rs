use super::*;

use battler::Battler;

use godot::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrops {
    pub item_id: StringName,
    pub odds: u8,
}

/// All the information the game needs to know about an enemy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub battler: Battler,
    pub drops: ItemDrops,
}

impl EnemyData {
    pub fn from_registry(id: StringName) -> &'static Self {
        unwrap_fmt!(REGISTRIES.enemies.get(&id), "Enemy ID not found: {}", id)
    }
}
