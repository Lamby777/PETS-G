//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use crate::prelude::*;

use super::dbox::DialogBox;

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

    pub fn show_dialog(&self) {
        let input = Input::singleton();
        let dummy = input.is_action_just_pressed("ui_accept".into());

        if dummy {
            let mut dbox_gd = self.dbox_scene.instantiate_as::<DialogBox>();

            dbox_gd.set_name("Beesechurger".into());
            self.node
                .get_window()
                .unwrap()
                .add_child(dbox_gd.clone().upcast());

            // simple stuff like this is why I love this language
            {
                let mut dbox = dbox_gd.bind_mut();
                dbox.set_txts(PChar::ETHAN.into(), "Hello, world!".into());
                dbox.pop_up()
            }
        }
    }
}

#[godot_api]
impl Node2DVirtual for DBoxInterface {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            dbox_scene: load::<PackedScene>("res://scenes/dialog.tscn"),
        }
    }
}
