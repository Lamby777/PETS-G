//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, Area2DVirtual};
use godot::prelude::*;

use crate::dialogue::DialogueAction;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct InteractionZone {
    #[base]
    node: Base<Area2D>,

    #[export]
    name: GodotString,

    action: DialogueAction,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        godot_print!("Interacted!");

        // TODO
    }
}

#[godot_api]
impl Area2DVirtual for InteractionZone {
    fn init(node: Base<Area2D>) -> Self {
        Self {
            node,
            name: "".into(),
            action: DialogueAction::End,
        }
    }
}
