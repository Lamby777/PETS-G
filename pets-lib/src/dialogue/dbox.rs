//!
//! Dialog box class for menus and dialogue text
//!

use dialogical::Speaker::{self, *};
use dialogical::{Interaction, Page};
use dialogical::{Metaline, Metaline::*, PageMeta};

use godot::engine::tween::TransitionType;
use godot::engine::{IPanelContainer, InputEvent, PanelContainer, RichTextLabel, Tween};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::DBoxInterface;

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

#[derive(Clone)]
pub struct MetaPair<T> {
    pub temporary: T,
    pub permanent: T,
}

impl<T> MetaPair<T> {
    pub fn clone(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            temporary: v.clone(),
            permanent: v.clone(),
        }
    }
}

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct DialogBox {
    #[base]
    node: Base<PanelContainer>,

    // state for the current interaction
    current_ix: Option<Interaction>,
    current_page_number: usize,
    speaker: MetaPair<Speaker>,
    vox: MetaPair<String>,
    tween: Option<Gd<Tween>>,
    active: bool,

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

    pub fn is_active(&self) -> bool {
        self.active
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
        let spk = spk_display(&self.speaker.temporary);
        self.set_txts(spk, msg);
    }

    /// Updates the speaker and vox based on the given page metadata
    pub fn update_meta(&mut self, meta: &PageMeta) {
        Self::match_meta(&mut self.speaker, &meta.speaker);
        Self::match_meta(&mut self.vox, &meta.vox);
    }

    /// helper method for `update_meta`
    ///
    /// matches over a `Metaline` to update a field depending on
    /// whether it's pageonly, permanent, or nochange
    fn match_meta<'a, T: Clone>(field: &'a mut MetaPair<T>, meta_field: &'a Metaline<T>) {
        field.temporary = match meta_field {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                field.permanent = v.clone();
                v
            }
            NoChange => &field.permanent,
        }
        .clone();
    }

    #[func]
    pub fn do_draw(&mut self) {
        // I THINK this clone is fine, probably RC'd
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
    }

    pub fn cancel_tween(&mut self) {
        if let Some(tween) = &mut self.tween {
            tween.stop()
        }
    }

    pub fn tween_into_view(&mut self, up: bool) -> Gd<Tween> {
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
            .set_trans(DBOX_TWEEN_TRANS)
            .unwrap();

        self.active = up;
        self.tween = Some(y_tween.clone());
        y_tween
    }
}

#[godot_api]
impl IPanelContainer for DialogBox {
    fn init(node: Base<PanelContainer>) -> Self {
        Self {
            node,
            spk_txt: "Cherry".into(),
            msg_txt: "[wave amp=50 freq=6]Hello, World![/wave]".into(),

            active: false,
            tween: None,
            current_ix: None,
            current_page_number: 0,
            speaker: MetaPair::clone(Speaker::Narrator),
            vox: MetaPair::clone(DEFAULT_VOX.to_owned()),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept".into()) {
            if !self.active {
                return;
            }

            // go to next page
            let ix = self.current_ix.as_ref().unwrap();
            self.current_page_number += 1;

            if self.current_page_number >= ix.pages.len() {
                self.tween_into_view(false);

                return;
            }

            godot_print!("going to page {}", self.current_page_number);
            self.goto_page(self.current_page_number);
            self.do_draw();
        }
    }
}
