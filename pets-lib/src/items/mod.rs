//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::common::*;
use godot::prelude::*;

mod inv;
pub use inv::{Equipment, Inventory};

/// A single item definition, stored in a vector for lookup.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// Things that describe what the item does or is
    ///
    /// This may be used for weaknesses/resistances, sorting purposes,
    /// shopkeeper price calculations, etc.
    pub attributes: Vec<ItemAttribute>,

    /// The category of the item. This affects how you can use it in-game.
    pub category: ItemCat,
}

// more derive spam :D
/// The category an item belongs to
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCat {
    Equipment {
        /// The category of equipment this item belongs to (weapon, armor, accessory)
        category: EquipmentCat,

        /// Stat offsets that the item applies when equipped
        offsets: LeveledStats,

        /// Only characters in this list can equip this
        equippable_by: Option<Vec<String>>,
    },
    AmmoBox {
        category: AmmoCat,
        uses: u32,
    },
    Consumable {
        func: String,
        uses: u32,
        in_overworld: bool,
        in_battle: bool,
    },
    Key,
}

/// The category an equippable item belongs to
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentCat {
    Weapon,
    Armor,
    Accessory,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmmoCat {
    Small,
    Medium,
    Large,
    Bolts,
    Shells,
    Comp, // i forgot what this was for tbh lol
}

impl Item {
    pub fn from_registry(id: impl Into<StringName>) -> &'static Self {
        let sn = id.into();
        unwrap_fmt!(REGISTRIES.items.get(&sn), "Item ID not found: {}", sn)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemAttribute {
    Rare,
    Expensive,
    Cheap,

    Melee,
    Ranged,
    Explosive,
}
