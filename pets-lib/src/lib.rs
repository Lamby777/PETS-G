//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(variant_count)]
#![feature(try_blocks)]
#![feature(let_chains)]

use godot::engine::Engine;
use godot::prelude::*;

use dialogue::autoload::DBoxInterface;
use stats::stats_interface::StatsInterface;

mod battle;
mod consts;
mod dialogue;
mod items;
mod limiq;
mod macros;
mod main_menu;
mod stats;
mod world;

mod prelude {
    pub use crate::items::*;
    pub use crate::limiq::*;
    pub use crate::macros::*;
    pub use crate::stats::*;

    pub use crate::dialogue::autoload::DBoxInterface;
    pub use crate::dialogue::ix_map;

    pub use crate::world::interaction::manager::InteractionManager;

    // is this bad practice? no clue and idc honestly
    // it's convenient with no real caveat, therefore...
    pub use anyhow::{bail, Result};
    pub use serde::{Deserialize, Serialize};
    pub use std::cell::RefCell;
    pub use std::collections::{HashMap, HashSet};
    pub use std::fmt::{Debug, Display};
    pub use std::ops::Deref;
    pub use std::rc::Rc;
}

struct PetsLib;

#[gdextension]
unsafe impl ExtensionLibrary for PetsLib {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();

            let gd = StatsInterface::alloc_gd();
            engine.register_singleton("Stats".into(), gd.upcast());

            let gd = DBoxInterface::alloc_gd();
            engine.register_singleton("DBox".into(), gd.upcast());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            for autoload_name in ["Stats", "DBox"] {
                engine.unregister_singleton(autoload_name.into());
            }
        }
    }
}
