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

/// Gets the node at a given path, without having to pass in self.node
#[macro_export]
macro_rules! node_at {
    ($path:expr) => {
        godot::engine::Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
            .get_root()
            .unwrap()
            .get_node_as($path)
    };

    ($path:expr, $type:ty) => {
        godot::engine::Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
            .get_root()
            .unwrap()
            .get_node_as::<$type>($path)
    };
}
