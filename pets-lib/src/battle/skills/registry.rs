use super::*;
use crate::prelude::*;

use std::io::Read as _;
use std::sync::OnceLock;

use godot::engine::file_access::ModeFlags;
use godot::engine::DirAccess;
use godot::prelude::*;

pub static SKILL_REGISTRY: OnceLock<Vec<Box<dyn Skill>>> = OnceLock::new();

/// Find all the modded skills from modded registries.
///
/// # Memory
///
///  This function leaks memory. It only runs once, and it's for
///  mods anyway, so it shouldn't be a big deal. I just typically
///  put a warning label on any function that leaks memory, so here
///  it is. You've been warned.
pub fn find_modded_skills() -> Vec<Box<dyn Skill>> {
    // make the folder in case it doesn't exist yet
    DirAccess::open("user://".into())
        .unwrap()
        .make_dir("mod-skills".into());

    let Some(mut dir) = DirAccess::open("user://mod-skills/".into()) else {
        godot_warn!(
            "Could not open `mod-skills`, no modded skills were loaded."
        );
        return vec![];
    };

    dir.get_files()
        .to_vec()
        .into_iter()
        .filter_map(|v| read_skill_registry(&v.to_string()))
        .flatten()
        .collect()
}

pub fn read_skill_registry(path: &str) -> Option<Vec<Box<dyn Skill>>> {
    let mut file = GFile::open(path, ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;
    if content.len() > 100_000_000 {
        godot_warn!("{} mod skills file too large! (over 100MB, wtf?)", path);
        godot_warn!("None of your modded skills will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;
    let res: Vec<Box<dyn Skill>> = ribbons::unwrap_fmt!(
        serde_json::from_str(&content),
        "skills file {} has wrong JSON contents",
        path
    );

    Some(res)
}

/// Initializes `SKILL_REGISTRY` by scanning for vanilla and
/// modded skill registries and combining the list of skills.
pub fn load_skill_registry() {
    let mut dir =
        DirAccess::open("res://assets/skillregistries".into()).unwrap();

    // scan the vanilla skills folder
    let mut skills = dir
        .get_files()
        .to_vec()
        .into_iter()
        .map(|fname| {
            godot_print!("Reading vanilla skill registry: {}", fname);
            let path = format!("res://assets/skillregistries/{}", fname);
            let skills = read_skill_registry(&path).expect(
                "Error loading vanilla skills. THIS IS A BUG, please report!",
            );

            godot_print!("Vanilla registry {} read!", fname);
            skills
        })
        .flatten()
        .collect::<Vec<_>>();

    // scan for modded skill paths
    skills.extend(find_modded_skills());

    godot_print!("Finished reading skill registries.\n\n");

    SKILL_REGISTRY.set(skills).unwrap();
    godot_print!("`SKILL_REGISTRY` initialized!");
}
