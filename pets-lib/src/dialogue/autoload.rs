//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::CanvasLayer;
use godot::prelude::*;

use super::dbox::DialogBox;
use crate::consts::dialogue::*;
use crate::prelude::*;

/// Autoload class for easy management of dialog boxes
#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct DBoxInterface {
    base: Base<Object>,
}

impl Autoload for DBoxInterface {
    const AUTOLOAD_NAME: &'static str = "DBox";
}

#[godot_api]
impl DBoxInterface {
    #[func]
    pub fn start_ix(&mut self, ix_id: String) {
        let ix = ix_map().get(&ix_id);
        let ix = unwrap_fmt!(
            ix,
            "Could not find interaction \"{}\" in the interaction map",
            ix_id,
        );

        let mut dbox = self.dbox();
        let mut dbox = dbox.bind_mut();
        dbox.set_ix(ix.clone());
        dbox.tween_into_view(true);
    }

    #[func]
    pub fn has_active_dbox(&self) -> bool {
        let ui_layer = current_scene().get_node_as::<Node>(UI_LAYER_NAME);

        ui_layer
            .try_get_node_as::<DialogBox>(DBOX_NODE_NAME)
            .map_or(false, |dbox| dbox.bind().is_active())
    }

    #[func]
    pub fn dbox(&self) -> Gd<DialogBox> {
        let ui_layer =
            current_scene().get_node_as::<CanvasLayer>(UI_LAYER_NAME);

        let mut dbox = ui_layer
            .try_get_node_as::<DialogBox>(DBOX_NODE_NAME)
            .expect("no dbox found");

        dbox.bind_mut().cancel_tween();
        dbox
    }
}
