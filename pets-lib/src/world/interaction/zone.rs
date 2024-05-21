//!
//! Area that allows the player to start
//! an interaction when within range
//!

use godot::engine::tween::TransitionType;
use godot::engine::{Area2D, ColorRect, IArea2D, ShaderMaterial};
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

    fn tp_player_to_beacon(&self, target: &NodePath) {
        fade_black(true);

        let mut pcb = PlayerCB::singleton();
        {
            let mut pcb = pcb.bind_mut();
            if pcb.tpbeacon_debounce {
                return;
            }

            pcb.tpbeacon_debounce = true;
        }

        let target_node = self.base().get_node_as::<Node2D>(target.clone());
        let target_pos = target_node.get_global_position();

        set_timeout(TP_BEACON_BLACK_IN, move || {
            // after the screen is black, teleport the player
            PlayerCB::singleton().set_global_position(target_pos);

            set_timeout(TP_BEACON_BLACK_HOLD, || {
                // when it's time to fade the black away, do it.
                fade_black(false);

                set_timeout(TP_BEACON_BLACK_OUT, || {
                    // finally, reset the debounce when the black is gone
                    PlayerCB::singleton().bind_mut().tpbeacon_debounce = false;
                });
            });
        });
    }
}

fn fade_black(visible: bool) {
    let node = current_scene().get_node_as::<ColorRect>("%BeaconFade");

    let material = node.get_material().unwrap().cast::<ShaderMaterial>();
    let material_id = material.instance_id();

    let callable = Callable::from_fn("set_shader_value", move |args| {
        let mut material = Gd::<ShaderMaterial>::from_instance_id(material_id);
        material.set_shader_parameter("opacity".into(), args[0].clone());

        // ...
        Ok(Variant::nil())
    });

    let (end_value, tween_time) = match visible {
        true => (1.0, TP_BEACON_BLACK_IN),
        false => (0.0, TP_BEACON_BLACK_OUT),
    };

    let start_value = material.get_shader_parameter("opacity".into());

    tween_method(
        callable,
        start_value,
        end_value.to_variant(),
        tween_time,
        TransitionType::QUAD,
    )
    .unwrap();
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
