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

/// Get the root from anywhere without having to pass in self.node
#[macro_export]
macro_rules! godot_root {
    () => {
        $crate::godot_tree!().get_root().unwrap()
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
