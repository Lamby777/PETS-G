use super::*;
use crate::common::*;
use crate::util::registry::*;

use std::io::Read as _;
use std::sync::OnceLock;

use godot::classes::file_access::ModeFlags;
use godot::classes::DirAccess;
use godot::prelude::*;

pub static SKILL_REGISTRY: OnceLock<HashMap<String, Box<dyn Skill>>> =
    OnceLock::new();

/// Initializes `SKILL_REGISTRY` by scanning for vanilla and
/// modded skill registries and combining the list of skills.
pub fn load_skill_registry() {
    let mut skills = find_vanilla("skillregistries");
    skills.extend(find_modded("skills"));
    SKILL_REGISTRY.set(skills).unwrap();
}
