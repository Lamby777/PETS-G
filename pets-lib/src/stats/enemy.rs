use super::*;

use battler::Battler;

use godot::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrop {
    pub item_id: StringName,
    pub odds: u8,
}

/// All the information the game needs to know about an enemy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub init_battler: Battler,
    pub drops: Vec<ItemDrop>,
}

impl EnemyData {
    pub fn from_registry(id: StringName) -> &'static Self {
        unwrap_fmt!(REGISTRIES.enemies.get(&id), "Enemy ID not found: {}", id)
    }
}
