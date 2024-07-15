use super::*;
use crate::prelude::*;
use crate::util::registry::*;

use std::io::Read as _;
use std::sync::OnceLock;

use godot::engine::file_access::ModeFlags;
use godot::engine::DirAccess;
use godot::prelude::*;

pub static SKILL_REGISTRY: OnceLock<HashMap<String, Box<dyn Skill>>> =
    OnceLock::new();

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
            let skills = read_registry(&path).expect(
                "Error loading vanilla skills. THIS IS A BUG, please report!",
            );

            godot_print!("Vanilla registry {} read!", fname);
            skills
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    // scan for modded skill paths
    skills.extend(find_modded());

    godot_print!("Finished reading skill registries.\n\n");

    SKILL_REGISTRY.set(skills).unwrap();
    godot_print!("`SKILL_REGISTRY` initialized!");
}
