//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::engine::{Control, InputEvent, RichTextLabel};
use godot::prelude::*;

use crate::consts::dialogue::INTERACT_PROMPT_HEIGHT_OFFSET;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct InteractionManager {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "Prompt"))]
    prompt: OnReady<Gd<Control>>,

    /// All interaction zones the player is inside
    zones: Vec<Gd<InteractionZone>>,
}

#[godot_api]
impl InteractionManager {
    #[func]
    pub fn singleton() -> Gd<Self> {
        World::singleton().get_node_as("%InteractionManager")
    }

    #[func]
    pub fn register_zone(&mut self, obj: Gd<InteractionZone>) {
        self.zones.push(obj);
    }

    #[func]
    pub fn unregister_zone(&mut self, obj: Gd<InteractionZone>) {
        self.zones.retain(|v| *v != obj);
    }

    /// Sorts the zones by distance from the player
    pub fn sort_zones_by_distance(&mut self) {
        let pcb_pos = pcb().get_position();

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

    fn set_prompt_text(&mut self, action: GString) {
        let key = format!("IZ_INTERACT_{}", action);

        self.prompt
            .get_node_as::<RichTextLabel>("%Action")
            .set_text(tr(key));
    }

    fn move_prompt_to_zone(&mut self, zone: Gd<InteractionZone>) {
        let custom_path = zone.bind().get_prompt_location();

        let pos = if custom_path.is_empty() {
            zone.get_global_position()
                + Vector2::new(0.0, -INTERACT_PROMPT_HEIGHT_OFFSET)
        } else {
            zone.get_node_as::<Node2D>(custom_path)
                .get_global_position()
        };

        self.prompt.set_global_position(pos);
    }
}

#[godot_api]
impl INode2D for InteractionManager {
    fn ready(&mut self) {
        self.prompt.hide();
        self.base_mut().show();
    }

    fn process(&mut self, _delta: f64) {
        self.sort_zones_by_distance();

        let Some(zone) = self.closest_zone() else {
            // if no zones, hide the prompt
            self.prompt.hide();
            return;
        };

        // move the prompt to the zone
        let prompt_tr_key = zone.bind().get_prompt_translation_key();
        self.set_prompt_text(prompt_tr_key);
        self.prompt.show();
        self.move_prompt_to_zone(zone);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept".into()) {
            {
                if !pcb().bind().can_move() {
                    // can't interact with stuff if you're
                    // not allowed to move.
                    return;
                }
            }

            if let Some(mut zone) = self.closest_zone() {
                zone.bind_mut().interact();
            }
        }
    }
}
