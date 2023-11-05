//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

// TODO remove this soon
#![allow(dead_code)]
#![feature(variant_count)]

use godot::engine::Engine;
use godot::prelude::*;

use dialogue::autoload::DBoxInterface;
use stats::stats_interface::StatsInterface;

mod battle;
mod dialogue;
mod items;
mod limiq;
mod macros;
mod main_menu;
mod stats;
mod world;

mod prelude {
    pub use crate::macros::*;

    // maybe make a data structures module if i end up making more?
    pub use crate::limiq::*;

    // item stuff, probably useful everywhere
    pub use crate::items::*;

    pub use crate::world::interaction::manager::InteractionManager;

    // stats stuff
    pub use crate::stats::pchars::PChar;
    pub use crate::stats::savefiles::SaveFile;
    pub use crate::stats::statcalc::{CharStatCalcs, StatCalcFn, StatCalcList};
    pub use crate::stats::stats_interface::StatsInterface;
    pub use crate::stats::*;

    // is this bad practice? no clue and idc honestly
    // it's convenient with no real caveat, therefore...
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

            let gd: Gd<StatsInterface> = Gd::new_default();
            engine.register_singleton("Stats".into(), gd.upcast());

            let gd: Gd<DBoxInterface> = Gd::new_default();
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
