//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::{RichTextLabel, Theme, Tween, Window};
use godot::prelude::*;

pub use crate::change_scene;

/// takes a bbcode string and operates on the node's text, either:
///
/// adds the bbcode to the start if `active` is true...
/// otherwise removes the first `bbcode.len()` characters
///
/// If it's deleting the start of your string when you don't want it to,
/// you probably incorrectly passed `false` for `active`.
pub fn bbcode_toggle(mut node: Gd<RichTextLabel>, bbcode: &str, active: bool) {
    let old_text = node.get_text();

    let new_text = if active {
        format!("{}{}", bbcode, old_text)
    } else {
        let st: String = old_text.into();
        st[bbcode.len()..].to_owned()
    };

    node.set_text(new_text.into());
}

/// shorthand to do some tweeneroonies :3
#[must_use = "`None` = failed to create tween"]
pub fn tween<NP, V>(
    mut node: Gd<Node>,
    property: NP,
    start_value: Option<V>,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Option<Gd<Tween>>
where
    NP: Into<NodePath>,
    V: ToGodot,
{
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

    Some(tween)
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

#[macro_export]
macro_rules! change_scene {
    ($scene:expr) => {
        godot_tree().change_scene_to_file(concat!("res://scenes/", $scene, ".tscn").into())
    };
}
