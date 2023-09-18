//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::stats::CharStats;

#[derive(Debug)]
pub struct Item {
    categories: Vec<ItemCategory>,

    /// difference in stats after equipping
    equip_offset: CharStats,
}

#[derive(Debug)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Key,
    Consumable,
}
