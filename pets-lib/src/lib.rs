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

mod dialogue;
// use dialogue::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for BattleEngine {
    fn init(node: Base<Node2D>) -> Self {
        // Prints to the Godot console
        godot_print!("Hello, world!");
        Self { node }
    }
}
