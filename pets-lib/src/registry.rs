//!
//! Exports all registries in the game.
//!
//! A registry is a hash map used to keep track of every possible instance
//! of a certain thing that can exist in the game. For example, registries
//! are used for defining every type of inventory item in the game.
//!
//! At first, they get populated with the hard-coded vanilla game stuff,
//! and then all modded data is loaded into them as well. Vanilla registries
//! are in the same format as modded registries, so modded registries have
//! pretty much first-class support.
//!
//! TODO: removed modded registry loader while reorganizing. add back later.
//!

use crate::battle::skills::Skill;
use crate::common::*;

use std::collections::HashMap;
use std::io::Read as _;
use std::sync::{LazyLock, Mutex};

use godot::classes::file_access::ModeFlags;
use godot::classes::DirAccess;
use godot::prelude::*;
use serde::de::DeserializeOwned;
use string_interner::symbol::SymbolU32;

trait Registry {
    type Value;

    /// Add a new item to the registry. This should never error.
    /// NOTE: might need to rm `mut` and use interior mutability later...
    fn append(&mut self, key: impl AsRef<str>, value: Self::Value);

    /// Convert the registry into a serializable format.
    /// This should "undo" any string interning. `Symbol`s become `String`s.
    fn pre_serialize() -> HashMap<String, Self::Value>;
}

impl<T> Registry for HashMap<SymbolU32, T> {
    type Value = T;

    fn append(&mut self, key: impl AsRef<str>, value: Self::Value) {
        let symbol = INTERNER.get_or_intern(key);

        if let Some(_) = self.insert(symbol, value) {
            panic!();
        }
    }

    fn pre_serialize() -> HashMap<String, Self::Value> {
        todo!()
    }
}

// A static instance of the collection
pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(|| Registries {
    items: find_vanilla_registries("items"),
    skills: find_vanilla_registries("skills"),
    chars: find_vanilla_registries("chars"),
});

pub struct Registries {
    pub items: HashMap<SymbolU32, Item>,
    pub skills: HashMap<SymbolU32, Box<dyn Skill>>,
    pub chars: HashMap<SymbolU32, Mutex<CharData>>,
}

pub fn read_registry_file<T>(path: &str) -> Option<HashMap<SymbolU32, T>>
where
    T: DeserializeOwned + Serialize,
{
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;

    let content = String::from_utf8(content).ok()?;
    let res: HashMap<SymbolU32, T> = unwrap_fmt!(
        serde_json::from_str(&content),
        "registry {path} failed to parse",
    );

    Some(res)
}

pub fn find_vanilla_registries<T>(folder_name: &str) -> HashMap<SymbolU32, T>
where
    T: DeserializeOwned + Serialize,
{
    let folder_path = format!("res://registries/{folder_name}");
    godot_print!("Reading vanilla registries at `{}`.", folder_path);

    let res = DirAccess::open(&folder_path)
        .unwrap()
        .get_files()
        .to_vec()
        .into_iter()
        .flat_map(|fname| {
            let path = format!("{folder_path}/{fname}");
            godot_print!("Reading vanilla registry: {}", path);
            let content = read_registry_file(&path).expect(
                "Error loading a vanilla registry. THIS IS A BUG!! Please report!",
            );

            godot_print!("Vanilla registry {} read!", path);
            content
        })
        .collect::<HashMap<_, _>>();

    godot_print!("Success!\n\n");
    res
}
