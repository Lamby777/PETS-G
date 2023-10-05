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
use stats::state::StatsInterface;

mod battle;
mod dialogue;
mod items;
mod stats;

mod prelude {
    // TODO probably put dialog box opener method here later

    // item stuff, probably useful everywhere
    pub use crate::items::*;

    // stats stuff
    pub use crate::stats::pchars::PChar;
    pub use crate::stats::savefiles::SaveFile;
    pub use crate::stats::statcalc::{CharStatCalcs, StatCalcFn, StatCalcList};
    pub use crate::stats::state::StatsInterface;
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
            let gd: Gd<StatsInterface> = Gd::new_default();
            let object = gd.upcast::<Object>();

            Engine::singleton().register_singleton("Stats".into(), object);
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().unregister_singleton("Stats".into());
        }
    }
}
