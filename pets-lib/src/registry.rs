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
use std::sync::LazyLock;

use godot::classes::file_access::ModeFlags;
use godot::classes::DirAccess;
use godot::prelude::*;
use serde::de::DeserializeOwned;

type Registry<T> = HashMap<String, T>;

// A static instance of the collection
pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(|| Registries {
    items: find_vanilla_registries("items"),
    skills: find_vanilla_registries("skills"),
});

pub struct Registries {
    pub items: Registry<Item>,
    pub skills: Registry<Box<dyn Skill>>,
}

pub fn read_registry_file<T>(path: &str) -> Option<Registry<T>>
where
    T: DeserializeOwned + Serialize,
{
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;

    let content = String::from_utf8(content).ok()?;
    let res: Registry<T> = unwrap_fmt!(
        serde_json::from_str(&content),
        "registry {path} failed to parse",
    );

    Some(res)
}

pub fn find_vanilla_registries<T>(folder_name: &str) -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    let folder_path = format!("res://assets/{folder_name}");
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
