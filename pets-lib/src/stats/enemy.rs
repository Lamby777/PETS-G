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
    pub id: StringName,
    pub battler: Rc<RefCell<Battler>>,
}

impl EnemyData {
    pub fn new_from_eid(id: &StringName) -> Self {
        // TODO: this should load from the registry, add an enemy registry
        // when i'm finally done with character shit
        Self {
            id: id.clone(),
            battler: Default::default(),
        }
    }
}
