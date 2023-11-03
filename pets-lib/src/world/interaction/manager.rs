//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::engine::{Node2D, Node2DVirtual, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;
use crate::world::interaction::zone::InteractionZone;
use crate::world::playercb::PlayerCB;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InteractionManager {
    #[base]
    node: Base<Node2D>,
    prompt: Option<Gd<RichTextLabel>>,

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
        // TODO clone big bad >:3
        self.zones = self
            .zones
            .clone()
            .into_iter()
            .filter(|v| *v != obj)
            .collect();
    }

    pub fn prompt_shown(&mut self, hidden: bool) {
        let prompt = self.prompt.as_mut().unwrap();
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

    pub fn sort_zones(&mut self) {
        let mut tree = self.node.get_tree().unwrap();
        let pcb = tree.get_first_node_in_group("playercb".into()).unwrap();
        let pcb = pcb.cast::<PlayerCB>();
        let pcb_pos = { pcb.get_position() };

        // TODO optimize sorting
        self.zones.sort_by(|a, b| {
            let a = a.get_global_position();
            let b = b.get_global_position();
            let a = a.distance_squared_to(pcb_pos);
            let b = b.distance_squared_to(pcb_pos);
            a.partial_cmp(&b).unwrap()
        });
    }
}

#[godot_api]
impl Node2DVirtual for InteractionManager {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            zones: vec![],
            prompt: None,
        }
    }

    fn ready(&mut self) {
        self.prompt = Some(self.node.get_node_as("Prompt"));
    }

    fn process(&mut self, _delta: f64) {
        let no_zones_found = self.zones.len() == 0;
        self.prompt_shown(no_zones_found);
        if no_zones_found {
            return;
        }

        self.sort_zones();

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_accept".into()) {
            let zone = self.zones[0].bind_mut();
            zone.interact();
        }
    }
}
