//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::prelude::*;

use godot::engine::DirAccess;
use std::cell::OnceCell;

mod inv;

pub use inv::ItemList;

pub const ITEM_REGISTRY: OnceCell<HashMap<String, Item>> = OnceCell::new();

/// Load a list of files from one of many registry files
pub fn load_item_registry_part(filename: &str) -> HashMap<String, Item> {
    let content = "STFU RUSTC, `todo!()` SHOULD SILENCE WARNINGS, NOT MAKE EM!";

    ribbons::unwrap_fmt!(
        toml::from_str(content),
        "items file {} has wrong TOML contents",
        filename
    )
}

pub fn load_item_registry(scan_folders: &[&str]) {
    let items = HashMap::new();

    let load_items = |path: &str| {
        let mut new_items = load_item_registry_part(filename);
        items.extend(new_items.drain());
    };

    // scan the vanilla items
    {
        let _dir =
            DirAccess::open("res://assets/itemregistries/{}".into()).unwrap();
    }

    // scan for modded item paths
    for _dir in scan_folders {
        // scan read files without res://
    }

    ITEM_REGISTRY.set(items).expect("item registry already set");
}

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
