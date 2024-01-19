//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::{RichTextLabel, Theme, Tween, Window};
use godot::prelude::*;

mod unwrap;
pub use unwrap::*;

/// takes a bbcode string and prepends or removes it from the label text
pub fn bbcode_toggle(mut node: Gd<RichTextLabel>, bbcode: &str, active: bool) {
    // TODO maybe there's a way to slice directly from the GString?
    // waiting for a reply to my "noob question" thread on discord...
    let old_text = node.get_text().to_string();
    let new_text = prefix_mod(&old_text, bbcode, active);

    node.set_text(new_text.into());
}

/// adds `prefix` to the start of `target` if `active` is true...
/// otherwise removes the first `prefix.len()` characters
///
/// panics if `target` is shorter than `prefix`.
/// you also need to make sure you don't call it with `false`
/// if the prefix isn't already there. be careful with this function...
pub fn prefix_mod(target: &str, prefix: &str, active: bool) -> String {
    if active {
        format!("{}{}", prefix, target)
    } else {
        let st: String = target.into();
        st[prefix.len()..].to_owned()
    }
}

/// shorthand to do some tweeneroonies :3
pub fn tween<NP, V>(
    mut node: Gd<Node>,
    property: NP,
    start_value: Option<V>,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    NP: Into<NodePath>,
    V: ToGodot,
{
    let res: Option<_> = try {
        let mut tween = node.create_tween()?;

        let mut property = tween
            .tween_property(
                node.clone().upcast(),
                property.into(),
                end_value.to_variant(),
                time,
            )?
            .set_trans(trans)?;

        if let Some(start_value) = start_value {
            property.from(start_value.to_variant())?;
        }

        tween
    };

    res.ok_or(())
}

pub fn default_theme() -> Gd<Theme> {
    load("res://themes/theme_deft.tres")
}

pub fn godot_tree() -> Gd<SceneTree> {
    godot::engine::Engine::singleton()
        .get_main_loop()
        .unwrap()
        .cast()
}

#[allow(unused)]
pub fn godot_root() -> Gd<Window> {
    godot_tree().get_root().unwrap()
}

pub fn current_scene() -> Gd<Node> {
    godot_tree().get_current_scene().unwrap()
}

pub use crate::change_scene;
#[macro_export]
macro_rules! change_scene {
    ($scene:expr) => {
        godot_tree().change_scene_to_file(concat!("res://scenes/", $scene, ".tscn").into())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_mod() {
        assert_eq!(prefix_mod("hello", "world", true), "worldhello");
        assert_eq!(prefix_mod("worldhello", "world", false), "hello");
    }
}
