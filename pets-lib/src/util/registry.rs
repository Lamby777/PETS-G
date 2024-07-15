use crate::prelude::*;
use std::collections::HashMap;

use godot::engine::file_access::ModeFlags;
use godot::engine::DirAccess;
use godot::prelude::*;
use io::Read as _;

type Registry<T> = HashMap<String, T>;

pub fn read_registry<T>(path: &str) -> Option<Registry<T>>
where
    T: DeserializeOwned + Serialize,
{
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;
    if content.len() > 100_000_000 {
        godot_warn!("{} mod skills file too large! (over 100MB, wtf?)", path);
        godot_warn!("None of your modded skills will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;
    let res: Registry<T> = ribbons::unwrap_fmt!(
        serde_json::from_str(&content),
        "skills file {} has wrong JSON contents",
        path
    );

    Some(res)
}

/// Find all the modded skills from modded registries.
///
/// # Memory
///
///  This function leaks memory. It only runs once, and it's for
///  mods anyway, so it shouldn't be a big deal. I just typically
///  put a warning label on any function that leaks memory, so here
///  it is. You've been warned.
pub fn find_modded<T>() -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    // make the folder in case it doesn't exist yet
    DirAccess::open("user://".into())
        .unwrap()
        .make_dir("mod-skills".into());

    let Some(mut dir) = DirAccess::open("user://mod-skills/".into()) else {
        godot_warn!(
            "Could not open `mod-skills`, no modded skills were loaded."
        );
        return HashMap::new();
    };

    dir.get_files()
        .to_vec()
        .into_iter()
        .filter_map(|v| read_registry::<T>(&v.to_string()))
        .flatten()
        .collect()
}
