//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, Area2DVirtual};
use godot::prelude::*;

use crate::dialogue::DialogueAction;
use crate::prelude::InteractionManager;
use crate::world::playercb::PlayerCB;

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
        // TODO
        godot_print!("Interacted!");
    }

    #[func]
    fn on_entered(&mut self, _body: Gd<PlayerCB>) {
        let mut im = InteractionManager::singleton();
        im.bind_mut().register_zone(self.node.to_godot().cast());
    }

    #[func]
    fn on_exited(&mut self, _body: Gd<PlayerCB>) {
        let mut im = InteractionManager::singleton();
        im.bind_mut().unregister_zone(self.node.to_godot().cast());
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
