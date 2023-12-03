//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::Tween;
use godot::prelude::*;

pub use crate::current_scene;
pub use crate::godot_root;
pub use crate::godot_tree;
pub use crate::node_at;

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
macro_rules! uninit {
    ($target:ty) => {
        unsafe {
            std::mem::transmute::<std::mem::MaybeUninit<$target>, $target>(
                std::mem::MaybeUninit::uninit().assume_init(),
            )
        }
    };
}

#[macro_export]
macro_rules! godot_tree {
    () => {
        godot::engine::Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
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

/// Gets the node at any given path
#[macro_export]
macro_rules! node_at {
    ($path:expr) => {
        $crate::godot_root!().get_node_as($path)
    };

    ($path:expr, $type:ty) => {
        $crate::godot_root!().get_node_as::<$type>($path)
    };
}
