//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

use crate::prelude::*;
use crate::world::playercb::PlayerCB;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct InteractionZone {
    #[base]
    node: Base<Area2D>,

    #[export]
    name: GString,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        let mut di = DBoxInterface::singleton();
        di.bind_mut().start_ix("Rodrick Sign #1".to_string());
    }

    #[func]
    fn on_entered(&mut self, body: Gd<Node2D>) {
        // body should inherit PlayerCB
        if let Err(_) = body.try_cast::<PlayerCB>() {
            return;
        }

        let mut im = InteractionManager::singleton();
        im.bind_mut().register_zone(self.base().clone().cast());
    }

    #[func]
    fn on_exited(&mut self, body: Gd<Node2D>) {
        // body should inherit PlayerCB
        if let Err(_) = body.try_cast::<PlayerCB>() {
            return;
        }

        let mut im = InteractionManager::singleton();
        im.bind_mut().unregister_zone(self.base().clone().cast());
    }
}

#[godot_api]
impl IArea2D for InteractionZone {
    fn ready(&mut self) {
        let mut node = self.base_mut();

        let enter_fn = node.callable("on_entered");
        let exit_fn = node.callable("on_exited");
        node.connect("body_entered".into(), enter_fn);
        node.connect("body_exited".into(), exit_fn);
    }
}
