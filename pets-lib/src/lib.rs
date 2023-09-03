/*
 * So this is where it all starts...
 *
 * Patiently awaiting the spaghetti-code horrors
 * that await me in this file in the future...
 *
 * - Cherry 9/2/2023 | <3
 */

// suppress unused warnings
#![allow(unused)]

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

mod battle_engine;
mod dialogue;
// use dialogue::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
