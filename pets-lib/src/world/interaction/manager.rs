//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::engine::{InputEvent, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct InteractionManager {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "Prompt"))]
    prompt_txt: OnReady<Gd<RichTextLabel>>,

    /// All interaction zones the player is inside
    zones: Vec<Gd<InteractionZone>>,
}

#[godot_api]
impl InteractionManager {
    #[func]
    pub fn register_zone(&mut self, obj: Gd<InteractionZone>) {
        self.zones.push(obj);
    }

    #[func]
    pub fn unregister_zone(&mut self, obj: Gd<InteractionZone>) {
        self.zones.retain(|v| *v != obj);
    }

    /// "ummm ackshually, this is not a singleton"
    pub fn singleton() -> Gd<Self> {
        current_scene().get_node_as("%InteractionManager")
    }

    /// Sorts the zones by distance from the player
    pub fn sort_zones_by_distance(&mut self) {
        let pcb_pos = PlayerCB::singleton().get_position();

        self.zones.sort_by(|zone_a, zone_b| {
            let a = zone_a.get_global_position().distance_squared_to(pcb_pos);
            let b = zone_b.get_global_position().distance_squared_to(pcb_pos);
            a.partial_cmp(&b).unwrap()
        });
    }

    /// Get the closest zone to the player
    /// Assumes the zones are already sorted
    ///
    /// Panics if there are no zones
    pub fn closest_zone(&mut self) -> Option<Gd<InteractionZone>> {
        self.zones.get(0).cloned()
    }
}

#[godot_api]
impl INode2D for InteractionManager {
    fn process(&mut self, _delta: f64) {
        self.sort_zones_by_distance();

        let Some(zone) = self.closest_zone() else {
            // if no zones, hide the prompt
            self.prompt_txt.hide();
            return;
        };

        // move the prompt to the zone
        self.prompt_txt.show();
        self.prompt_txt
            .set_position(zone.get_position() + Vector2::new(0.0, -50.0));
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept".into()) {
            {
                let pcb = PlayerCB::singleton();
                if !pcb.bind().can_move() {
                    // can't interact with stuff if you're
                    // not allowed to move.
                    return;
                }
            }

            if let Some(zone) = self.closest_zone() {
                zone.bind().interact();
            }
        }
    }
}
