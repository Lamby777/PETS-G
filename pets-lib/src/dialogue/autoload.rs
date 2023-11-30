//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use super::dbox::DialogBox;
use crate::prelude::*;

/// Autoload class for easy management of dialog boxes
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct DBoxInterface {
    #[base]
    node: Base<Node2D>,
    dbox_scene: Gd<PackedScene>,
}

/// Show a dialog box with the given speaker and message
/// usage: `show_dialog!("Cherry", "Hello, {}!", name, ...)`
#[macro_export]
macro_rules! show_dialog {
    ($speaker:expr, $($t:tt)*) => {{
        let msg = format!($($t)*);

        let dbox = crate::dialogue::autoload::DBoxInterface::singleton();
        dbox.bind().show_dialog($speaker.into(), msg.into());
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
    pub fn start_ix(&self, ix_id: String) {
        use dialogical::Metaline::*;
        use dialogical::Speaker::*;

        let ix = ix_map().get(&ix_id).unwrap_or_else(|| {
            panic!(
                "Could not find interaction \"{}\" in the interaction map",
                ix_id
            )
        });

        let page = ix.pages.get(0).unwrap();
        let spk = page.metadata.speaker.clone();

        let spk = match spk {
            PageOnly(v) | Permanent(v) => v,
            NoChange => todo!(),
        };

        let spk = match spk {
            Named(v) => v,
            Narrator => "".to_string(),
            Unknown => "???".to_string(),
        };

        // TODO multi-page stuff, don't just pop up twice
        let msg = page.content.clone();
        self.show_dialog(spk.into(), msg.into());
        // show_dialog!(spk, "{}", page.content.clone());
    }

    #[func]
    pub fn show_dialog(&self, spk: GString, msg: GString) {
        let mut dbox_gd = self.dbox_scene.instantiate_as::<DialogBox>();

        dbox_gd.set_name("Dialog".into());
        current_scene!()
            .get_node("UILayer".into())
            .expect("scene should have a UILayer")
            .add_child(dbox_gd.clone().upcast());

        // simple stuff like this is why I love this language
        {
            let mut dbox = dbox_gd.bind_mut();
            dbox.set_txts(spk, msg);
            dbox.do_draw();
            dbox.pop_up()
        }
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
