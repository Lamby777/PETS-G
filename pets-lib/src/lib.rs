//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

#![allow(dead_code)]
use godot::engine::Engine;
use godot::prelude::*;

mod battle;
mod dialogue;
mod items;
mod stats;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Core {
            use stats::state::StatsInterface;

            let gd: Gd<StatsInterface> = Gd::new_default();
            let object = gd.share().upcast::<Object>();

            Engine::singleton().register_singleton("Stats".into(), object);
        }
    }
}
