//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::prelude::*;

/// A single item definition, stored in item hashtable for lookup.
// Or maybe just in a vector... and there can be a function
// that looks up the item by searching the vector for an Item
// with the correct `name` property?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// The category of the item. This affects how you can use it in-game.
    category: ItemCat,

    /// Things that describe what the item does or is
    ///
    /// This may be used for weaknesses/resistances, sorting purposes,
    /// shopkeeper price calculations, etc.
    attributes: Vec<ItemAttribute>,

    name: String,
    description: String,
}

// more derive spam :D
/// The category an item belongs to
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCat {
    Equipment(EquipmentCat, InherentStats),
    Key,
    Consumable,
}

/// The category an equippable item belongs to
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentCat {
    Weapon,
    Armor,
    Accessory,
}

impl Item {
    pub fn is_equipment(&self) -> bool {
        matches!(self.category, ItemCat::Equipment(..))
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

// implement for vector of items
pub trait ItemsList {
    /// Every item that can be equipped
    fn equipment(&self) -> impl Iterator<Item = &Item>;
    fn offsets(&self) -> impl Iterator<Item = &InherentStats>;
}

impl ItemsList for &[Item] {
    fn equipment(&self) -> impl Iterator<Item = &Item> {
        self.iter().filter(|i| i.is_equipment())
    }

    fn offsets(&self) -> impl Iterator<Item = &InherentStats> {
        use ItemCat::*;

        self.iter().filter_map(|i| match &i.category {
            Equipment(_, offsets) => Some(offsets),
            _ => None,
        })
    }
}
