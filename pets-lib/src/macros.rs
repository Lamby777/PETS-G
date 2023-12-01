//!
//! Macros for mostly general-purpose Godot/Rust stuff
//!

pub use crate::current_scene;
pub use crate::godot_root;
pub use crate::godot_tree;
pub use crate::node_at;

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
