//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::common::*;
use crate::util::registry::*;

use std::sync::OnceLock;

mod inv;
pub use inv::{Equipment, Inventory};

pub static ITEM_REGISTRY: OnceLock<HashMap<String, Item>> = OnceLock::new();

/// Initializes `ITEM_REGISTRY` by scanning for vanilla and
/// modded item registries and combining the list of items.
pub fn load_item_registry() {
    let mut items = find_vanilla("itemregistries");
    items.extend(find_modded("items"));
    ITEM_REGISTRY.set(items).unwrap();
}

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
        offsets: InherentStats,

        /// Only characters in this list can equip this
        equippable_by: Vec<PChar>,
    },
    AmmoBox {
        category: AmmoCat,
        uses: u32,
    },
    Consumable {
        func: String,
        uses: u32,
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
    Comp,
}

impl Item {
    pub fn from_registry(id: &str) -> &Item {
        unwrap_fmt!(
            ITEM_REGISTRY.get().unwrap().get(id),
            "Item ID not found: {}",
            id
        )
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
