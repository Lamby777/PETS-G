//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::{Area2D, ColorRect, IArea2D};
use godot::prelude::*;

use crate::consts::playercb::*;
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
            self.tp_player_to_beacon(target);
        }
    }

    #[func]
    fn on_entered(&mut self, _body: Gd<Node2D>) {
        if self.auto_interact {
            self.interact();
            return;
        }

        InteractionManager::try_singleton()
            .unwrap()
            .bind_mut()
            .register_zone(self.to_gd());
    }

    #[func]
    fn on_exited(&mut self, _body: Gd<Node2D>) {
        if self.auto_interact {
            return;
        }

        InteractionManager::try_singleton()
            .unwrap()
            .bind_mut()
            .unregister_zone(self.to_gd());
    }

    fn tp_player_to_beacon(&self, target: &NodePath) {
        let black = self.base().get_node_as::<ColorRect>("%BeaconFade");

        fade_black(black, true, TP_BEACON_BLACK_IN);

        {
            let mut pcb = pcb();
            let mut pcb = pcb.bind_mut();
            if pcb.tpbeacon_debounce {
                return;
            }

            pcb.tpbeacon_debounce = true;
        }

        let target_node = self.base().get_node_as::<Node2D>(target.clone());
        let target_pos = target_node.get_global_position();
        let black_id = self
            .base()
            .get_node_as::<ColorRect>("%BeaconFade")
            .instance_id();

        set_timeout(TP_BEACON_BLACK_IN, move || {
            // after the screen is black, teleport the player
            pcb().bind_mut().teleport(target_pos, None, false);

            set_timeout(TP_BEACON_BLACK_HOLD, move || {
                // when it's time to fade the black away, do it.
                let black = Gd::<ColorRect>::from_instance_id(black_id);
                fade_black(black, false, TP_BEACON_BLACK_OUT);

                set_timeout(TP_BEACON_BLACK_OUT, || {
                    // finally, reset the debounce when the black is gone
                    pcb().bind_mut().tpbeacon_debounce = false;
                });
            });
        });
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
