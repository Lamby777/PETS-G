//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::Tween;
use godot::prelude::*;

pub use crate::current_scene;
pub use crate::default_theme;
pub use crate::godot_tree;

#[allow(unused)]
pub use crate::godot_root;

/// shorthand to do some tweeneroonies :3
#[must_use = "`None` = failed to create tween"]
pub fn tween<NP, V>(
    mut node: Gd<Node>,
    property: NP,
    from: Option<V>,
    target: V,
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
            target.to_variant(),
            time,
        )?
        .set_trans(trans)?;

    if let Some(from) = from {
        property.from(from.to_variant())?;
    }

    Some(tween)
}

#[macro_export]
macro_rules! default_theme {
    () => {
        load::<godot::engine::Theme>("res://themes/theme_deft.tres")
    };
}

#[macro_export]
macro_rules! godot_tree {
    () => {
        godot::engine::Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<godot::engine::SceneTree>()
    };
}

#[macro_export]
macro_rules! godot_root {
    () => {
        $crate::godot_tree!().get_root().unwrap()
    };
}

#[macro_export]
macro_rules! current_scene {
    () => {
        $crate::godot_tree!().get_current_scene().unwrap()
    };
}
