use godot::classes::Engine;
use godot::obj::NewAlloc;
use godot::prelude::*;

/// Trait for anything that has only 1 existing node..
pub trait Singleton: Inherits<Object> + NewAlloc {
    fn singleton() -> Gd<Self>;
}

/// Trait for autoloaded classes
/// Makes it more convenient to get the singleton instance
pub trait GodotAutoload: Singleton {
    const AUTOLOAD_NAME: &str;

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

impl<T: GodotAutoload> Singleton for T {
    /// Get a shared ref to use in other nodes
    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(Self::AUTOLOAD_NAME.into())
            .unwrap()
            .cast()
    }
}
