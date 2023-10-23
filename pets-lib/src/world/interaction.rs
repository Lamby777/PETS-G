//!
//! Area that allows the player to start
//! an interaction when within range
//!

use crate::prelude::*;
use godot::prelude::*;

use godot::engine::{Area2D, Area2DVirtual};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct InteractionZone {
    #[base]
    node: Base<Area2D>,
    si: Gd<StatsInterface>,
}

#[godot_api]
impl Area2DVirtual for InteractionZone {
    fn init(node: Base<Area2D>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),
        }
    }

    // fn ready(&mut self) {}
}
