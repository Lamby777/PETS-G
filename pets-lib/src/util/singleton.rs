use godot::engine::Engine;
use godot::obj::NewAlloc;
use godot::prelude::*;

/// Trait for autoloaded classes
/// Makes it more convenient to get the singleton instance
pub trait Autoload: GodotClass + Inherits<Object> + NewAlloc {
    const AUTOLOAD_NAME: &'static str;

    /// Get a shared ref to use in other nodes
    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(Self::AUTOLOAD_NAME.into())
            .unwrap()
            .cast()
    }

    /// Register the singleton with the engine
    fn register() {
        let gd = Self::new_alloc().upcast();
        let name = Self::AUTOLOAD_NAME.into();
        Engine::singleton().register_singleton(name, gd);
    }

    fn unregister() {
        let name = Self::AUTOLOAD_NAME.into();
        Engine::singleton().unregister_singleton(name);
    }
}
