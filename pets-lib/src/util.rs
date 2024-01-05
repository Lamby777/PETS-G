//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::Theme;
use godot::engine::Tween;
use godot::engine::Window;
use godot::prelude::*;

pub use crate::change_scene;

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
