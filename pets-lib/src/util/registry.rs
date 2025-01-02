use crate::common::*;
use std::collections::HashMap;

use godot::classes::file_access::ModeFlags;
use godot::classes::DirAccess;
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
        godot_warn!("{} mod file too large! (over 100MB, wtf?)", path);
        godot_warn!("None of your modded skills will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;
    let res: Registry<T> = unwrap_fmt!(
        serde_json::from_str(&content),
        "skills file {} has wrong JSON contents",
        path
    );

    Some(res)
}

/// Find all the modded skills from modded registries.
pub fn find_modded<T>(registry_name: &str) -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    godot_print!("Reading modded {} registries.", registry_name);

    // make the folder in case it doesn't exist yet
    DirAccess::open("user://").unwrap().make_dir("modded");

    DirAccess::open("user://modded")
        .unwrap()
        .make_dir(registry_name);

    let Some(mut dir) =
        DirAccess::open(&format!("user://modded/{}", registry_name))
    else {
        godot_warn!(
            "Could not open `/modded/{0}`, no modded {0} were loaded.",
            registry_name
        );
        return HashMap::new();
    };

    let res = dir
        .get_files()
        .to_vec()
        .into_iter()
        .filter_map(|v| read_registry::<T>(&v.to_string()))
        .flatten()
        .collect();

    godot_print!("Finished reading modded {} registries.\n\n", registry_name);
    res
}

pub fn find_vanilla<T>(folder_name: &str) -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    godot_print!("Reading vanilla `{}`.", folder_name);
    let folder_path = format!("res://assets/{}", folder_name);

    let res = DirAccess::open(&folder_path)
        .unwrap()
        .get_files()
        .to_vec()
        .into_iter()
        .flat_map(|fname| {
            let path = format!("{}/{}", folder_path, fname);
            godot_print!("Reading vanilla registry: {}", path);
            let content = read_registry(&path).expect(
                "Error loading vanilla registry. THIS IS A BUG, please report!",
            );

            godot_print!("Vanilla registry {} read!", path);
            content
        })
        .collect::<HashMap<_, _>>();

    godot_print!("Successfully read vanilla `{}`!\n\n", folder_name);
    res
}
