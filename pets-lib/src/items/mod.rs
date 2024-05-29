//!
//! This module has stuff required for implementing items.
//! For inventory-related types, check out `inv.rs`
//!

use crate::prelude::*;

use std::cell::OnceCell;
use std::io::Read as _;

use godot::engine::file_access::ModeFlags;
use godot::engine::DirAccess;
use godot::prelude::*;

mod inv;

pub use inv::ItemList;

pub const ITEM_REGISTRY: OnceCell<Vec<Item>> = OnceCell::new();

/// Find all the modded items from modded registries.
///
/// # Memory
///
///  This function leaks memory. It only runs once, and it's for
///  mods anyway, so it shouldn't be a big deal. I just typically
///  put a warning label on any function that leaks memory, so here
///  it is. You've been warned.
pub fn find_modded_items() -> Vec<Item> {
    // make the folder in case it doesn't exist yet
    DirAccess::open("user://".into())
        .unwrap()
        .make_dir("mod-items".into());

    let Some(mut dir) = DirAccess::open("user://mod-items/".into()) else {
        println!("Could not open `mod-items`, no modded items were loaded.");
        return vec![];
    };

    dir.get_files()
        .to_vec()
        .into_iter()
        .filter_map(|v| read_item_registry(&v.to_string()))
        .flatten()
        .collect()
}

pub fn read_item_registry(path: &str) -> Option<Vec<Item>> {
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;
    if content.len() > 100_000_000 {
        godot_warn!("{} mod_items.txt is too large! (over 100MB, wtf?)", path);
        godot_warn!("None of your modded items will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;
    ribbons::unwrap_fmt!(
        toml::from_str(&content),
        "items file {} has wrong TOML contents",
        path
    )
}

/// Initializes `ITEM_REGISTRY` by scanning for vanilla and
/// modded item registries and combining the list of items.
pub fn load_item_registry() {
    let table = vec![Item {
        category: ItemCat::Equipment {
            category: EquipmentCat::Weapon,
            offsets: InherentStats {
                max_hp: 0,
                max_energy: 0,
                attack: 0,
                defense: 0,
                speed: 0,
                stability: 0,
                delta: 0,
                epsilon: 0,
                lambda: Some(0),
                max_mana: Some(0),
            },
        },

        attributes: vec![ItemAttribute::Melee, ItemAttribute::Blade],
        name: "Sword".into(),
        description: "A sharp sword.".into(),
    }];
    let example = toml::to_string(&table).unwrap();
    println!("{}", example);

    let mut dir =
        DirAccess::open("res://assets/itemregistries".into()).unwrap();

    // scan the vanilla items folder
    let mut items = dir
        .get_files()
        .to_vec()
        .into_iter()
        .map(|file| {
            println!("Reading vanilla item registry: {}", file);
            read_item_registry(&file.to_string()).expect(
                "Error loading vanilla items. THIS IS A BUG, please report!",
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    // scan for modded item paths
    items.extend(find_modded_items());

    ITEM_REGISTRY
        .set(items)
        .expect("item registry already set. this is a bug!");
}

/// A single item definition, stored in a vector for lookup.
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
