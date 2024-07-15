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
    // scan the vanilla skills folder
    let mut skills = find_vanilla("skillregistries");

    // scan for modded skill paths
    skills.extend(find_modded("skills"));

    godot_print!("Finished reading skill registries.\n\n");

    SKILL_REGISTRY.set(skills).unwrap();
    godot_print!("`SKILL_REGISTRY` initialized!");
}
