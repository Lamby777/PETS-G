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

    amogus: i32,
}

#[godot_api]
impl StatsInterface {
    #[func]
    pub fn get_amogus(&self) -> i32 {
        self.amogus
    }

    #[func]
    pub fn set_amogus(&mut self) {
        self.amogus += 1;
    }
}

#[godot_api]
impl Node2DVirtual for StatsInterface {
    fn init(node: Base<Node2D>) -> Self {
        Self { node, amogus: 0 }
    }
}
