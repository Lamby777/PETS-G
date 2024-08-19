//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::common::*;
use crate::util::registry::*;

use std::sync::OnceLock;

mod inv;
pub use inv::{Inventory, ItemList};

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
        category: EquipmentCat,
        offsets: InherentStats,
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
    pub fn is_equipment(&self) -> bool {
        matches!(self.category, ItemCat::Equipment { .. })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemAttribute {
    Rare,
    Expensive,
    Cheap,

    Melee,
    Blade,

    Ranged,
    Firearm,

    Explosive,
    Grenade,
}
