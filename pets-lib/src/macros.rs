//!
//! Macros for mostly general-purpose Godot/Rust stuff
//!

pub use crate::current_scene;
pub use crate::godot_root;
pub use crate::godot_tree;
pub use crate::node_at;
pub use crate::show_dialog;

/// Show a dialog box with the given speaker and message
/// usage: `show_dialog!("Cherry", "Hello, {}!", name, ...)`
///
/// Don't use this in actual game code. It's kinda messy,
/// and more just meant for quick testing. Should probably
/// delete it later, but I'm keeping it for now.
#[macro_export]
macro_rules! show_dialog {
    ($speaker:expr, $($t:tt)*) => {{
        let msg = format!($($t)*);

        let dbox = crate::dialogue::autoload::DBoxInterface::singleton();
        dbox.bind().show_dialog($speaker.into(), msg.into());
    }};
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
