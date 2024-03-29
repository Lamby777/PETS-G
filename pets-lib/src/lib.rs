//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

// This is 80 characters, just for reference for setting IDE zoom and formatting

#![feature(variant_count)]
#![feature(let_chains)]
#![feature(lazy_cell)]
#![feature(try_blocks)]
#![feature(generic_arg_infer)]
#![feature(int_roundings)]

use godot::prelude::*;

use prelude::*;

mod battle;
mod consts;
mod dialogue;
mod functions;
mod items;
mod limiq;
mod main_menu;
mod singleton;
mod stats;
mod util;
mod world;
mod wrapped;

#[cfg(any())]
mod llm;

#[allow(unused)]
mod prelude {
    pub use crate::consts::type_aliases::*;

    pub use crate::items::*;
    pub use crate::limiq::*;
    pub use crate::stats::*;
    pub use crate::util::*;

    pub use crate::battle::BattleEngine;
    pub use crate::dialogue::{ix_map, DBoxInterface};
    pub use crate::functions::FnInterface;
    pub use crate::world::{
        InteractionManager, InteractionZone, MusicZone, PlayerCB, World,
    };

    pub use crate::singleton::Autoload;
    pub use crate::wrapped::Wrapped;

    // is this bad practice? no clue and idc honestly
    // it's convenient with no real caveat, therefore...
    pub use anyhow::{anyhow, Result};
    pub use derived_deref::{Deref, DerefMut};
    pub use indoc::indoc;
    pub use rand::Rng;
    pub use ribbons::unwrap_fmt;
    pub use serde::{Deserialize, Serialize};

    pub use std::cell::RefCell;
    pub use std::collections::{HashMap, HashSet};
    pub use std::convert::Infallible;
    pub use std::fmt::{self, Debug, Display};
    pub use std::io;
    pub use std::path::{Path, PathBuf};
    pub use std::rc::Rc;
}

struct PetsLib;

#[gdextension]
unsafe impl ExtensionLibrary for PetsLib {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        libdx::foreach_static!(
            [
                StatsInterface,
                DBoxInterface,
                FnInterface,
            ] => Autoload, register
        );
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        libdx::foreach_static!(
            [
                StatsInterface,
                DBoxInterface,
                FnInterface,
            ] => Autoload, unregister
        );
    }
}
