use godot::engine::Engine;
use godot::prelude::*;

///
/// Trait for autoloaded classes
/// Makes it more convenient to get the singleton instance
///
pub trait Autoload: GodotClass + Inherits<Object> {
    const AUTOLOAD_NAME: &'static str;

    /// Get a shared ref to use in other nodes
    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(Self::AUTOLOAD_NAME.into())
            .unwrap()
            .cast()
    }
}
