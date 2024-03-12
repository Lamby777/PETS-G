//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::prelude::*;

/// A single item definition, stored in item hashtable for lookup.
// Or maybe just in a vector... and there can be a function
// that looks up the item by searching the vector for an Item
// with the correct `name` property?
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    categories: Vec<ItemCategory>,

    /// difference in stats after equipping
    pub equip_offset: Option<InherentStats>,

    name: String,
    description: String,
}

impl Item {
    pub fn is_equipment(&self) -> bool {
        self.equip_offset.is_some()
    }
}

// more derive spam :D
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Key,
    Consumable,
}

// implement for vector of items
trait ItemsList {
    /// Every item that can be equipped
    fn equipment(&self) -> impl Iterator<Item = &Item>;
    fn offsets(&self) -> impl Iterator<Item = &InherentStats>;
}

impl ItemsList for &[Item] {
    fn equipment(&self) -> impl Iterator<Item = &Item> {
        self.iter().filter(|i| i.is_equipment())
    }

    fn offsets(&self) -> impl Iterator<Item = &InherentStats> {
        self.equipment().map(|i| i.equip_offset.as_ref().unwrap())
    }
}
