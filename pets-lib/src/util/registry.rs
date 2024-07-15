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
pub fn find_modded<T>(registry_name: &str) -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    // make the folder in case it doesn't exist yet
    DirAccess::open("user://".into())
        .unwrap()
        .make_dir("modded".into());

    DirAccess::open("user://modded".into())
        .unwrap()
        .make_dir(registry_name.into());

    let Some(mut dir) =
        DirAccess::open(format!("user://modded/{}", registry_name).into())
    else {
        godot_warn!(
            "Could not open `/modded/{0}`, no modded {0} were loaded.",
            registry_name
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

pub fn find_vanilla<T>(folder_name: &str) -> Registry<T>
where
    T: DeserializeOwned + Serialize,
{
    let folder_path = format!("res://assets/{}", folder_name);

    DirAccess::open(folder_path.clone().into())
        .unwrap()
        .get_files()
        .to_vec()
        .into_iter()
        .map(|fname| {
            let path = format!("{}/{}", folder_path, fname);
            godot_print!("Reading vanilla registry: {}", path);
            let content = read_registry(&path).expect(
                "Error loading vanilla registry. THIS IS A BUG, please report!",
            );

            godot_print!("Vanilla registry {} read!", path);
            content
        })
        .flatten()
        .collect::<HashMap<_, _>>()
}
