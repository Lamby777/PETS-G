use battler::Battler;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrops {
    pub item: Item,
    pub odds: u8,
}

/// All the information the game needs to know about a character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub id: String,
    pub battler: Rc<RefCell<Battler>>,
}

impl EnemyData {
    pub fn from_id(id: EnemyID) -> Self {
        // TODO load from file
        Self {
            id: id.to_string(),
            battler: Default::default(),
        }
    }
}
