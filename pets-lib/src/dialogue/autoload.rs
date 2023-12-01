//!
//! Singleton for accessing player stats in GDScript.
//!

use dialogical::Metaline::*;
use dialogical::Speaker::{self, *};
use dialogical::{Interaction, Page};
use godot::engine::Engine;
use godot::prelude::*;

use super::dbox::DialogBox;
use crate::consts::dialogue::*;
use crate::prelude::*;

/// Autoload class for easy management of dialog boxes
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DBoxInterface {
    #[base]
    node: Base<Node2D>,
    dbox_scene: Gd<PackedScene>,
}

#[godot_api]
impl DBoxInterface {
    /// Get a shared ref to the singleton to store in other node structs
    pub fn singleton() -> Gd<DBoxInterface> {
        Engine::singleton()
            .get_singleton("DBox".into())
            .unwrap()
            .cast()
    }

    #[func]
    pub fn start_ix(&mut self, ix_id: String) {
        let ix = ix_map().get(&ix_id).unwrap_or_else(|| {
            panic!(
                "Could not find interaction \"{}\" in the interaction map",
                ix_id
            )
        });

        let mut dbox = self.instantiate_dbox();

        {
            let mut dbox = dbox.bind_mut();
            dbox.set_ix(ix.clone());
            dbox.do_draw();
            dbox.tween_into_view(true)
        }
    }

    #[func]
    pub fn instantiate_dbox(&self) -> Gd<DialogBox> {
        let mut dbox = self.dbox_scene.instantiate_as::<DialogBox>();
        dbox.set_name(DBOX_NODE_NAME.into());

        let mut ui_layer = current_scene!()
            .get_node("UILayer".into())
            .expect("scene should have a UILayer");

        // check if a box already exists
        if ui_layer.has_node(DBOX_NODE_NAME.into()) {
            let node = ui_layer.get_node(DBOX_NODE_NAME.into()).unwrap();
            ui_layer.remove_child(node);
        }

        ui_layer.add_child(dbox.clone().upcast());

        dbox
    }
}

#[godot_api]
impl INode2D for DBoxInterface {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            dbox_scene: load::<PackedScene>("res://scenes/dialog.tscn"),
        }
    }
}
