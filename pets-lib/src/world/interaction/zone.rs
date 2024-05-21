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
    base: Base<Area2D>,

    #[export]
    interaction_id: GString,

    #[export]
    /// The beacon this one sends you to
    beacon_target: NodePath,

    #[export]
    auto_interact: bool,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        let ix_id = self.interaction_id.to_string();
        if !ix_id.is_empty() {
            start_ix(ix_id);

            // A zone can't be both a beacon AND an interaction
            return;
        }

        let target = &self.beacon_target;
        if !target.is_empty() {
            let target_node = self.base().get_node_as(target.clone());
            tp_to_beacon(target_node);

            return;
        }
    }

    #[func]
    fn on_entered(&mut self, _body: Gd<Node2D>) {
        if self.auto_interact {
            self.interact();
            return;
        }

        InteractionManager::singleton()
            .bind_mut()
            .register_zone(self.to_gd());
    }

    #[func]
    fn on_exited(&mut self, _body: Gd<Node2D>) {
        if self.auto_interact {
            return;
        }

        InteractionManager::singleton()
            .bind_mut()
            .unregister_zone(self.to_gd());
    }
}

fn tp_to_beacon(target: Gd<Node2D>) {
    let pos = target.get_global_position();
    PlayerCB::singleton().set_global_position(pos);
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
