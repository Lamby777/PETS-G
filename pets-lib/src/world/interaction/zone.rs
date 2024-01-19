//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct InteractionZone {
    #[base]
    node: Base<Area2D>,

    #[export]
    interaction_id: GString,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        let ix_id = self.interaction_id.to_string();

        let mut di = DBoxInterface::singleton();
        di.bind_mut().start_ix(ix_id);
    }

    #[func]
    fn on_enter_or_exit(&mut self, entered: bool) {
        let zone = self.base().clone().cast();
        let mut im = InteractionManager::singleton();
        let mut im = im.bind_mut();

        if entered {
            im.register_zone(zone);
        } else {
            im.unregister_zone(zone);
        }
    }

    #[func]
    fn on_entered(&mut self, _body: Gd<Node2D>) {
        self.on_enter_or_exit(true);
    }

    #[func]
    fn on_exited(&mut self, _body: Gd<Node2D>) {
        self.on_enter_or_exit(false);
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
