//!
//! Singleton for accessing player stats in GDScript.
//!

use dialogical::{Interaction, Speaker};
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

    // state for the current interaction
    current_ix: Option<Interaction>,
    current_page_number: usize,
    current_speaker: Speaker,
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
        // let vox = page.metadata.vox.clone();

        // TODO so many clones lol
        // prob move this to another get/set 2-in-1 function too
        let spk = match spk {
            PageOnly(v) => v,
            Permanent(v) => {
                self.current_speaker = v.clone();
                v
            }
            NoChange => self.current_speaker.clone(),
        };

        let spk = match spk {
            Named(ref v) => v,
            Narrator => NARRATOR_DISPLAYNAME,
            Unknown => UNKNOWN_DISPLAYNAME,
        };

        let msg = page.content.clone();
        self.show_dialog(spk.into(), msg.into());
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

            current_ix: None,
            current_page_number: 0,
            current_speaker: Speaker::Narrator,
        }
    }
}
