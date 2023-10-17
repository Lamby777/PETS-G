//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use super::dbox::DialogBox;

/// Autoload class for easy management of dialog boxes
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DBoxInterface {
    #[base]
    node: Base<Node2D>,
    dbox_scene: Gd<PackedScene>,
}

#[macro_export]
macro_rules! show_dialog {
    ($any_node:expr, $speaker:expr, $($t:tt)*) => {{
        let msg = format!($($t)*);

        // as long as the node is in the scene, this will work
        let root = $any_node.get_tree().unwrap().get_root().unwrap();

        let dbox = crate::dialogue::autoload::DBoxInterface::singleton();
        dbox.bind().show_dialog(root.upcast(), $speaker.into(), msg.into());
    }};
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
    pub fn show_dialog(&self, mut tree: Gd<Node>, spk: GodotString, msg: GodotString) {
        let mut dbox_gd = self.dbox_scene.instantiate_as::<DialogBox>();

        dbox_gd.set_name("Dialog".into());
        tree.add_child(dbox_gd.clone().upcast());

        // simple stuff like this is why I love this language
        {
            let mut dbox = dbox_gd.bind_mut();
            dbox.set_txts(spk, msg);
            dbox.pop_up()
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
