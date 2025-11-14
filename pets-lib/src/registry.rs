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

// Nothing should ever be removed from a registry! Only push new entries.
// NOTE: Should probably say don't touch registries directly-- expose a function
// specifically for modders to push their own stuff onto the registry without
// being able to remove anything.
pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(|| Registries {
    items: find_vanilla_registries("items"),
    skills: find_vanilla_registries("skills"),
    chars: find_vanilla_registries("chars"),
    enemies: find_vanilla_registries("enemies"),
});

pub struct Registries {
    pub items: HashMap<StringName, Item>,
    pub skills: HashMap<StringName, Box<dyn Skill>>,
    pub chars: HashMap<StringName, ConstCharData>,
    pub enemies: HashMap<StringName, EnemyData>,
}

pub fn read_registry_file<T>(path: &str) -> Option<HashMap<StringName, T>>
where
    T: DeserializeOwned + Serialize,
{
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;

    let content = String::from_utf8(content).ok()?;
    let res: HashMap<StringName, T> = unwrap_fmt!(
        serde_json::from_str(&content),
        "registry {path} failed to parse",
    );

    Some(res)
}

pub fn find_vanilla_registries<T>(folder_name: &str) -> HashMap<StringName, T>
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
        // TODO: remove logging if it works
        .filter(|fname| {dbg!(&fname); !dbg!(should_ignore_registry(fname))})
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

/// Predicate for a registry being ignored.
/// This list of conditions is like a gitignore for registries pretty much.
///
/// Document each step well so it's obvious what's a bug and what's intended.
fn should_ignore_registry(fname: &GString) -> bool {
    // No *.jsonnet files, they get transpiled to *.json
    fname.ends_with(".jsonnet")
}
