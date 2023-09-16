//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct StatsInterface {
    #[base]
    node: Base<Node2D>,
}

impl StatsInterface {
    // pub fn new() -> Self {
    //     Self { node: Base::new() }
    // }
}

#[godot_api]
impl Node2DVirtual for StatsInterface {
    fn init(node: Base<Node2D>) -> Self {
        Self { node }
    }
}
