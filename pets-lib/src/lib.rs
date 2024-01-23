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
#![feature(generic_arg_infer)]

use godot::engine::Engine;
use godot::prelude::*;

use prelude::{DBoxInterface, FnInterface, StatsInterface};

mod battle;
mod consts;
mod dialogue;
mod functions;
mod items;
mod limiq;
mod main_menu;
mod stats;
mod util;
mod world;
mod wrapped;

mod prelude {
    pub use crate::items::*;
    pub use crate::limiq::*;
    pub use crate::stats::*;
    pub use crate::util::*;

    pub use crate::dialogue::autoload::DBoxInterface;
    pub use crate::functions::FnInterface;

    pub use crate::dialogue::ix_map;
    pub use crate::world::interaction::manager::InteractionManager;
    pub use crate::wrapped::Wrapped;

    // is this bad practice? no clue and idc honestly
    // it's convenient with no real caveat, therefore...
    pub use ribbons::unwrap_fmt;
    pub use serde::{Deserialize, Serialize};

    pub use std::cell::RefCell;
    pub use std::collections::{HashMap, HashSet};
    pub use std::ops::{Deref, DerefMut};
    pub use std::rc::Rc;

    #[allow(unused_imports)]
    pub use std::fmt::{Debug, Display};
}

struct PetsLib;

macro_rules! register_singletons {
    ($($struct:ident => $name:expr),*) => {
        $(
            let gd = $struct::new_alloc();
            Engine::singleton().register_singleton($name.into(), gd.upcast());
        )*
    };
}

const SINGLETON_NAMES: &[&str] = &["Stats", "DBox", "Functions"];

#[gdextension]
unsafe impl ExtensionLibrary for PetsLib {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            register_singletons! {
                StatsInterface => "Stats",
                DBoxInterface => "DBox",
                FnInterface => "Functions"
            };
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            for autoload_name in SINGLETON_NAMES {
                engine.unregister_singleton(autoload_name.into());
            }
        }
    }
}
