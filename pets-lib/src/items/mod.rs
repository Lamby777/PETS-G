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
    equip_offset: CharStats,

    name: String,
    description: String,
}

// are you serious m8
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Key,
    Consumable,
}
