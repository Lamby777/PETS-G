//!
//! So this is where it all starts...
//!
//! Patiently awaiting the spaghetti-code horrors
//! that await me in this file in the future...
//!
//! - Cherry 9/2/2023 | <3
//!

use godot::prelude::*;

mod battle;
mod dialogue;
mod items;
mod stats;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
