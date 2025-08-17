//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry, 2023-09-02 | <3
//!

// This is 80 characters, just for reference for setting IDE zoom and formatting

#![feature(try_blocks)]
// #![warn(clippy::missing_docs_in_private_items)]

use godot::prelude::*;

use common::*;

mod consts;
mod util;

mod battle;
mod dialogue;
mod items;
mod registry;
mod stats;
mod title_screen;
mod world;

/// Module for all common stuff used throughout the codebase.
/// You'll often see `use crate::common::*;` at the top of most files.
mod common {
    pub use crate::consts::type_aliases::*;

    pub use crate::items::*;
    pub use crate::limiq::*;
    pub use crate::registry::*;
    pub use crate::stats::*;
    pub use crate::util::*;

    pub use crate::battle::{Affinities, BattleEngine};
    pub use crate::dialogue::DialogBox;
    pub use crate::world::{InteractionZone, PartyCB, World};

    pub use crate::choices::ChoiceAgent;
    pub use crate::singleton::{GodotAutoload, Singleton};

    // re-exports
    pub use nodi::midly;

    pub use anyhow::Result;
    pub use chrono::{Datelike, NaiveDate};
    pub use derived_deref::{Deref, DerefMut};
    // pub use indoc::indoc;
    pub use ribbons::unwrap_fmt;
    // pub use rand::Rng;
    pub use serde::{Deserialize, Serialize};

    pub use std::cell::RefCell;
    pub use std::collections::{HashMap, HashSet};
    pub use std::fmt::{self, Debug, Display};
    // pub use std::path::{Path, PathBuf};
    pub use std::rc::Rc;
    pub use std::thread;
}

/// The GDExtension library struct
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
            ] => GodotAutoload, register
        );
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }

        libdx::foreach_static!(
            [
                StatsInterface,
            ] => GodotAutoload, unregister
        );
    }
}
