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

    // state for the current interaction
    // TODO combine speaker and vox into a `PageMeta`
    current_ix: Option<Interaction>,
    current_page_number: usize,
    current_speaker: Speaker,
    current_vox: String,
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

    /// Takes a NAME metaline and updates the speaker accordingly
    pub fn update_spk(&mut self, page: &Page) -> String {
        let spk = match page.metadata.speaker.clone() {
            PageOnly(v) => v,
            Permanent(v) => {
                self.current_speaker = v.clone();
                v
            }
            NoChange => self.current_speaker.clone(),
        };

        match spk {
            Named(ref v) => v,
            Narrator => NARRATOR_DISPLAYNAME,
            Unknown => UNKNOWN_DISPLAYNAME,
        }
        .to_owned()
    }

    /// Takes a VOX metaline and updates the vox accordingly
    pub fn update_vox(&mut self, page: &Page) -> String {
        match page.metadata.vox.clone() {
            PageOnly(v) => v,
            Permanent(v) => {
                self.current_vox = v.clone();
                v
            }
            NoChange => self.current_vox.clone(),
        }
    }

    #[func]
    pub fn start_ix(&mut self, ix_id: String) {
        let ix = ix_map().get(&ix_id).unwrap_or_else(|| {
            panic!(
                "Could not find interaction \"{}\" in the interaction map",
                ix_id
            )
        });

        let page = ix.pages.get(0).unwrap();
        let spk = self.update_spk(page);

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
            current_vox: DEFAULT_VOX.to_owned(),
        }
    }
}
