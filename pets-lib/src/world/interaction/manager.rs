//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::engine::{Engine, Node2D, Node2DVirtual};
use godot::prelude::*;

use crate::world::interaction::zone::InteractionZone;
use crate::world::playercb::PlayerCB;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct InteractionManager {
    #[base]
    node: Base<Node2D>,

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

    pub fn singleton() -> Gd<InteractionManager> {
        Engine::singleton()
            .get_singleton("Interactions".into())
            .unwrap()
            .cast()
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
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.zones.len() == 0 {
            // TODO hide label
            return;
        }

        self.sort_zones();

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_accept".into()) {
            let zone = self.zones[0].bind_mut();
            zone.interact();
        }

        // TODO show label
    }
}
