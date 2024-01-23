use godot::engine::Engine;
use godot::prelude::*;

pub trait Singleton: GodotClass + Inherits<Object> {
    const SINGLETON_NAME: &'static str;

    fn singleton() -> Gd<Self> {
        Engine::singleton()
            .get_singleton(Self::SINGLETON_NAME.into())
            .unwrap()
            .cast()
    }
}
