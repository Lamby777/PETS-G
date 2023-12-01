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

/// Turn a Speaker into a displayable name
///
/// Either the name of the speaker or a special name
/// if it's a narrator or unknown speaker
pub fn spk_display(spk: &Speaker) -> String {
    match spk {
        Named(ref v) => v,
        Narrator => NARRATOR_DISPLAYNAME,
        Unknown => UNKNOWN_DISPLAYNAME,
    }
    .to_owned()
}

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
    permanent_speaker: Speaker,
    permanent_vox: String,

    // independent from any interaction-related stuff,
    // these are the actual strings that are displayed
    //
    // you can set these directly if you're doing something
    // that's not part of an interaction
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
    ///
    /// DON'T USE THIS FOR INTERACTIONS!!
    /// That's what `goto_page` is for.
    #[func]
    pub fn set_txts(&mut self, speaker: String, content: String) {
        self.spk_txt = speaker.into();
        self.msg_txt = content.into();
    }

    pub fn set_ix(&mut self, ix: Interaction) {
        self.current_ix = Some(ix);
        self.goto_page(0);
    }

    /// basically set_txts but for an interaction page
    pub fn goto_page(&mut self, pageno: usize) {
        let ix = self.current_ix.as_ref().unwrap().clone();
        let page = ix.pages.get(pageno).unwrap();

        self.update_meta(&page.metadata);
        let msg = page.content.clone();
        let spk = spk_display(&self.current_speaker);
        self.set_txts(spk, msg);
    }

    /// Takes a NAME metaline and updates the speaker accordingly
    pub fn update_meta(&mut self, meta: &PageMeta) {
        self.current_speaker = match meta.speaker {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.permanent_speaker = v.clone();
                v
            }
            NoChange => &self.permanent_speaker,
        }
        .clone();

        self.current_vox = match meta.vox {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.permanent_vox = v.clone();
                v
            }
            NoChange => &self.permanent_vox,
        }
        .to_owned();
    }

    #[func]
    pub fn do_draw(&mut self) {
        // I THINK this clone is fine, probably RC'd
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
    }

    pub fn tween_into_view(&mut self, up: bool) {
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
            permanent_speaker: Speaker::Narrator,
            permanent_vox: DEFAULT_VOX.to_owned(),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
