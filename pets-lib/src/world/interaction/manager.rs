//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::engine::{INode2D, InputEvent, Node2D, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;
use crate::world::interaction::zone::InteractionZone;
use crate::world::playercb::PlayerCB;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InteractionManager {
    #[base]
    node: Base<Node2D>,
    prompt_txt: Option<Gd<RichTextLabel>>,

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

    pub fn prompt_txt(&mut self) -> &mut Gd<RichTextLabel> {
        self.prompt_txt.as_mut().unwrap()
    }

    pub fn prompt_hidden(&mut self, hidden: bool) {
        let prompt = self.prompt_txt();
        if hidden {
            prompt.hide();
        } else {
            prompt.show();
        }
    }

    /// "ummm ackshually, this is not a singleton"
    pub fn singleton() -> Gd<InteractionManager> {
        // using this cool godot feature I just found...
        // you can set a node to be accessible with just its
        // name and a % prefix... nice for this sort of situation
        current_scene!().get_node_as("%InteractionManager")
    }

    /// Sorts the zones by distance from the player
    pub fn sort_zones_by_distance(&mut self) {
        let mut tree = self.node.get_tree().unwrap();
        let pcb = tree.get_first_node_in_group("playercb".into()).unwrap();
        let pcb = pcb.cast::<PlayerCB>();
        let pcb_pos = { pcb.get_position() };

        self.zones.sort_by(|a, b| {
            let a = a.get_global_position();
            let b = b.get_global_position();
            let a = a.distance_squared_to(pcb_pos);
            let b = b.distance_squared_to(pcb_pos);
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
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            zones: vec![],
            prompt_txt: None,
        }
    }

    fn ready(&mut self) {
        self.prompt_txt = Some(self.node.get_node_as("Prompt"));
    }

    fn process(&mut self, _delta: f64) {
        self.sort_zones_by_distance();

        if let Some(zone) = self.closest_zone() {
            self.prompt_hidden(false);
            self.prompt_txt()
                .set_position(zone.get_position() + Vector2::new(0.0, -50.0));
        } else {
            self.prompt_hidden(true);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept".into()) {
            let di = DBoxInterface::singleton();
            if di.bind().scene_has_active_dbox() {
                return;
            }

            if let Some(zone) = self.closest_zone() {
                zone.bind().interact();
            }
        }
    }
}
