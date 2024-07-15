//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::prelude::*;

use std::io::Read as _;
use std::sync::OnceLock;

use godot::engine::file_access::ModeFlags;
use godot::engine::DirAccess;
use godot::prelude::*;

mod inv;

pub use inv::{Inventory, ItemList};

pub static ITEM_REGISTRY: OnceLock<HashMap<String, Item>> = OnceLock::new();

/// Find all the modded items from modded registries.
pub fn find_modded_items() -> HashMap<String, Item> {
    // make the folder in case it doesn't exist yet
    DirAccess::open("user://".into())
        .unwrap()
        .make_dir("mod-items".into());

    let Some(mut dir) = DirAccess::open("user://mod-items/".into()) else {
        godot_warn!("Could not open `mod-items`, no modded items were loaded.");
        return HashMap::new();
    };

    dir.get_files()
        .to_vec()
        .into_iter()
        .filter_map(|v| read_item_registry(&v.to_string()))
        .flatten()
        .collect()
}

pub fn read_item_registry(path: &str) -> Option<HashMap<String, Item>> {
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;
    if content.len() > 100_000_000 {
        godot_warn!("{} mod items file too large! (over 100MB, wtf?)", path);
        godot_warn!("None of your modded items will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;
    ribbons::unwrap_fmt!(
        serde_json::from_str(&content),
        "items file {} has wrong JSON contents",
        path
    )
}

/// Initializes `ITEM_REGISTRY` by scanning for vanilla and
/// modded item registries and combining the list of items.
pub fn load_item_registry() {
    let mut dir =
        DirAccess::open("res://assets/itemregistries".into()).unwrap();

    // scan the vanilla items folder
    let mut items = dir
        .get_files()
        .to_vec()
        .into_iter()
        .map(|fname| {
            godot_print!("Reading vanilla item registry: {}", fname);
            let path = format!("res://assets/itemregistries/{}", fname);
            let items = read_item_registry(&path).expect(
                "Error loading vanilla items. THIS IS A BUG, please report!",
            );

            godot_print!("Vanilla registry {} read!", fname);
            items
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    // scan for modded item paths
    items.extend(find_modded_items());

    godot_print!("Finished reading item registries.\n\n");

    ITEM_REGISTRY.set(items).unwrap();
    godot_print!("`ITEM_REGISTRY` initialized!");
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
