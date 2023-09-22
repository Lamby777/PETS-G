//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::stats::CharStats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    categories: Vec<ItemCategory>,

    /// difference in stats after equipping
    equip_offset: CharStats,
}

// are you serious m8
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Key,
    Consumable,
}
