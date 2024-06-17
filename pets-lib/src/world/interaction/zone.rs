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
    /// The function (from the global functions table) that gets called
    function_name: GString,

    #[export]
    /// The scene the beacon belongs to
    beacon_room_name: GString,

    #[export]
    /// The beacon this one sends you to
    /// Path is relative to `Room/`
    beacon_target: GString,

    #[export]
    auto_interact: bool,

    #[export]
    #[init(default = "INTERACT".into())]
    prompt_translation_key: GString,
}

#[godot_api]
impl InteractionZone {
    #[func]
    pub fn interact(&self) {
        let ix_id = self.interaction_id.to_string();
        if !ix_id.is_empty() {
            start_ix(ix_id);

            // Interactions take priority, you can't have
            // functions or beacons at the same time as an interaction.
            // If you want to run code before an interaction, you can
            // use a function that later starts the interaction using
            // a call to `start_ix()`
            return;
        }

        let fn_id = self.function_name.to_string();
        if !fn_id.is_empty() {
            call_global(&fn_id).unwrap();

            // Functions and beacons can coexist.
            // Don't `return` here.
        }

        let target = &self.beacon_target;
        if !target.is_empty() {
            self.tp_player_to_beacon(target, &self.beacon_room_name);
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

    fn tp_player_to_beacon(&self, target: &GString, target_scene: &GString) {
        let target = target.to_string();
        let target_scene =
            Some(target_scene.to_string()).filter(|s| !s.is_empty());

        let black = World::singleton().get_node_as::<ColorRect>("%BeaconFade");
        fade_black(&black, true, TP_BEACON_BLACK_IN);

        {
            let mut pcb = pcb();
            let mut pcb = pcb.bind_mut();
            if pcb.tpbeacon_debounce {
                return;
            }

            pcb.tpbeacon_debounce = true;
        }

        let black_id = black.instance_id();

        let scene_id = target_scene.map(|s| {
            load::<PackedScene>(format!("res://scenes/rooms/{}.tscn", s))
                .instantiate()
                .unwrap()
                .instance_id()
        });

        set_timeout(TP_BEACON_BLACK_IN, move || {
            // once the screen is black, swap rooms if necessary
            if let Some(scene_id) = scene_id {
                let new_room_scene = Gd::from_instance_id(scene_id);
                World::singleton().bind_mut().change_room(new_room_scene);
            }

            let target_node =
                World::room().get_node_as::<Node2D>(target.clone());
            let target_pos = target_node.get_global_position();

            // after the screen is black, teleport the player
            // clear past positions if switching rooms
            let switching_rooms = scene_id.is_some();
            pcb().bind_mut().teleport(target_pos, None, switching_rooms);

            set_timeout(TP_BEACON_BLACK_HOLD, move || {
                // when it's time to fade the black away, do it.
                let black = Gd::<ColorRect>::from_instance_id(black_id);
                fade_black(&black, false, TP_BEACON_BLACK_OUT);

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
