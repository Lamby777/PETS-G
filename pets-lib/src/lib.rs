//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

#![feature(variant_count)]
#![feature(let_chains)]
#![feature(lazy_cell)]
#![feature(try_blocks)]

use godot::engine::Engine;
use godot::prelude::*;

use dialogue::autoload::DBoxInterface;
use stats::stats_interface::StatsInterface;

mod battle;
mod consts;
mod dialogue;
mod items;
mod limiq;
mod main_menu;
mod stats;
mod util;
mod world;
mod wrapped;

#[allow(unused_imports)]
mod prelude {
    pub use crate::items::*;
    pub use crate::limiq::*;
    pub use crate::stats::*;
    pub use crate::util::*;

    pub use crate::dialogue::autoload::DBoxInterface;
    pub use crate::dialogue::ix_map;
    pub use crate::world::interaction::manager::InteractionManager;
    pub use crate::wrapped::Wrapped;

    // is this bad practice? no clue and idc honestly
    // it's convenient with no real caveat, therefore...
    pub use anyhow::{bail, Result};
    pub use serde::{Deserialize, Serialize};
    pub use std::cell::RefCell;
    pub use std::collections::{HashMap, HashSet};
    pub use std::fmt::{Debug, Display};
    pub use std::ops::{Deref, DerefMut};
    pub use std::rc::Rc;
}

struct PetsLib;

#[gdextension]
unsafe impl ExtensionLibrary for PetsLib {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();

            let gd = StatsInterface::new_alloc();
            engine.register_singleton("Stats".into(), gd.upcast());

            let gd = DBoxInterface::new_alloc();
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
