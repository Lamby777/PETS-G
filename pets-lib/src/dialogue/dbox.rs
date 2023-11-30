//!
//! Dialog box class for menus and dialogue text
//!

use dialogical::Speaker::{self, *};
use dialogical::{Interaction, Page};
use dialogical::{Metaline::*, PageMeta};

use godot::engine::tween::TransitionType;
use godot::engine::{IPanelContainer, PanelContainer, RichTextLabel};
use godot::prelude::*;

use crate::consts::dialogue::*;

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct DialogBox {
    #[base]
    node: Base<PanelContainer>,

    // state for the current interaction
    current_ix: Option<Interaction>,
    current_page_number: usize,
    current_speaker: Speaker,
    current_vox: String,

    spk_txt: GString,
    msg_txt: GString,
}

#[godot_api]
impl DialogBox {
    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as("VSplit/SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as("VSplit/Content")
    }

    /// Sets the speaker and message text from strings
    #[func]
    pub fn set_txts(&mut self, speaker: String, content: String) {
        self.spk_txt = speaker.into();
        self.msg_txt = content.into();
    }

    pub fn set_ix(&mut self, ix: Interaction) {
        self.current_ix = Some(ix);
        self.goto_page(0);
    }

    pub fn goto_page(&mut self, pageno: usize) {
        let ix = self.current_ix.as_ref().unwrap().clone();
        let page = ix.pages.get(pageno).unwrap();
        let meta = &page.metadata;

        let spk = self.update_spk(meta);
        let vox = self.update_vox(meta);
        let msg = page.content.clone();

        self.spk_txt = spk.into();
    }

    /// Takes a NAME metaline and updates the speaker accordingly
    pub fn update_spk(&mut self, meta: &PageMeta) -> String {
        let spk = match meta.speaker {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.current_speaker = v.clone();
                v
            }
            NoChange => &self.current_speaker,
        };

        match spk {
            Named(ref v) => v,
            Narrator => NARRATOR_DISPLAYNAME,
            Unknown => UNKNOWN_DISPLAYNAME,
        }
        .to_owned()
    }

    /// Takes a VOX metaline and updates the vox accordingly
    pub fn update_vox(&mut self, meta: &PageMeta) -> String {
        match meta.vox {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.current_vox = v.clone();
                v
            }
            NoChange => &self.current_vox,
        }
        .to_owned()
    }

    #[func]
    pub fn do_draw(&mut self) {
        // I THINK this clone is fine, probably RC'd
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
    }

    #[func]
    pub fn pop_up(&mut self) {
        self.tween_into_view(true);
    }

    #[func]
    pub fn pop_down(&mut self) {
        self.tween_into_view(false);
    }

    fn tween_into_view(&mut self, up: bool) {
        let node = &mut self.node;
        let viewport_y = node.get_viewport_rect().size.y;
        let visible_y = viewport_y - node.get_size().y;

        let (tw_start, tw_end) = if up {
            (viewport_y, visible_y)
        } else {
            (visible_y, viewport_y)
        };

        let mut y_tween = node.create_tween().unwrap();
        y_tween
            .tween_property(
                node.clone().upcast(),
                "position:y".into(),
                Variant::from(tw_end),
                DBOX_TWEEN_TIME,
            )
            .unwrap()
            .from(Variant::from(tw_start))
            .unwrap()
            .set_trans(DBOX_TWEEN_TRANS);
    }
}

#[godot_api]
impl IPanelContainer for DialogBox {
    fn init(node: Base<PanelContainer>) -> Self {
        Self {
            node,
            spk_txt: "Cherry".into(),
            msg_txt: "[wave amp=50 freq=6]Hello, World![/wave]".into(),

            current_ix: None,
            current_page_number: 0,
            current_speaker: Speaker::Narrator,
            current_vox: DEFAULT_VOX.to_owned(),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
