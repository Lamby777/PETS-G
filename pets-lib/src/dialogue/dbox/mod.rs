//!
//! Dialog box class for menus and dialogue text
//!
//! Yes, an important distinction to make is that "dialogue" is
//! the speech between characters, and "dialog" is a window or
//! UI element that displays the dialogue. Don't get too confused!
//!

use dialogical::Speaker::{self, *};
use dialogical::{DialogueEnding, Interaction, Metaline, Metaline::*, PageMeta};

use godot::engine::{
    HBoxContainer, IPanelContainer, InputEvent, PanelContainer, RichTextLabel, Tween,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

// extra impls
mod choice;

// custom class for choice labels
mod dchoice;
use dchoice::DChoice;

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
    pub fn from_cloned(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            temporary: v.clone(),
            permanent: v,
        }
    }

    /// matches over a `Metaline` to update a field depending on
    /// whether it's pageonly, permanent, or nochange
    fn set_from<'a>(&mut self, meta: &'a Metaline<T>)
    where
        T: Clone,
    {
        self.temporary = match meta {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.permanent = v.clone();
                v
            }
            NoChange => &self.permanent,
        }
        .clone();
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
    selected_choice: Option<usize>,

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
    #[func]
    pub fn do_draw(&mut self) {
        // I THINK this clone is fine, probably RC'd
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
    }

    /// sets the speaker and message labels to the given page
    pub fn goto_page(&mut self, pageno: usize) {
        let ix = self.current_ix.as_ref().unwrap().clone();
        let page = ix.pages.get(pageno).unwrap();

        self.update_meta(&page.metadata);

        let msg = page.content.clone();
        let spk = spk_display(&self.speaker.temporary);
        self.spk_txt = spk.into();
        self.msg_txt = msg.into();
    }

    pub fn tween_into_view(&mut self, up: bool) -> Gd<Tween> {
        let node = self.base();
        let viewport_y = node.get_viewport_rect().size.y;

        let tw_end = viewport_y
            + if up {
                -node.get_size().y
            } else {
                DBOX_Y_BELOW_VIEWPORT
            };

        let y_tween = tween(
            node.clone().upcast(),
            "position:y",
            Some(node.get_position().y),
            tw_end,
            DBOX_TWEEN_TIME,
            DBOX_TWEEN_TRANS,
        );

        self.active = up;
        self.tween = y_tween.clone();
        y_tween.unwrap()
    }

    pub fn run_ix_ending(&mut self, ending: &DialogueEnding) {
        use dialogical::Label::*;
        use DialogueEnding::*;

        match ending {
            Choices(choices) => {
                self.recreate_choice_labels(choices);
                self.tween_choices_wave(true);
            }

            Label(Function(_label)) => {
                // TODO run the function
            }

            Label(Goto(_label)) => {
                // TODO "goto" the label
            }

            End => {
                self.tween_choices_wave(false);
            }
        }
    }
}

#[godot_api]
impl IPanelContainer for DialogBox {
    fn init(node: Base<PanelContainer>) -> Self {
        Self {
            node,
            spk_txt: "Cherry".into(),
            msg_txt: "[wave amp=50 freq=6]Hello, World![/wave]".into(),

            selected_choice: None,
            active: false,
            tween: None,
            current_ix: None,
            current_page_number: 0,
            speaker: MetaPair::from_cloned(Speaker::Narrator),
            vox: MetaPair::from_cloned(DEFAULT_VOX.to_owned()),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.active && event.is_action_pressed("ui_accept".into()) {
            // go to next page
            let ix = self.current_ix.as_ref().unwrap();
            let pagecount = ix.pages.len();

            self.current_page_number += 1;

            // if end of interaction, close the dialog
            if self.current_page_number >= pagecount {
                // TODO only do these if the ending is End
                // Function labels can close the dialog box on their own,
                // and Goto/Choice don't need to close the box anyway.
                // (Choice can still do it by using function labels tho)
                self.tween_into_view(false);
                self.current_page_number = 0;
            } else {
                if self.current_page_number == pagecount - 1 {
                    // if last page, we need to run the ix ending
                    let ix_ending = ix.ending.clone();
                    self.run_ix_ending(&ix_ending)
                }

                self.goto_page(self.current_page_number);
                self.do_draw();
            }

            // mark the input as handled
            self.base().get_viewport().unwrap().set_input_as_handled();
        }
    }
}

/// shorter methods that are sorta self-explanatory
/// moving 'em here to avoid clutter up above
impl DialogBox {
    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.base().get_node_as("VBox/SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.base().get_node_as("VBox/Content")
    }

    /// Get the container for choice labels
    fn choice_container(&self) -> Gd<HBoxContainer> {
        self.base().get_node_as("VBox/Choices")
    }

    /// If the dialog box is currently active
    ///
    /// Active means either tweening on-screen,
    /// OR on-screen and not tweening off-screen
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_ix(&mut self, ix: Interaction) {
        self.current_ix = Some(ix);
        self.goto_page(0);
    }

    /// Updates the speaker and vox based on the given page metadata
    pub fn update_meta(&mut self, meta: &PageMeta) {
        self.speaker.set_from(&meta.speaker);
        self.vox.set_from(&meta.vox);
    }

    pub fn cancel_tween(&mut self) {
        if let Some(tween) = &mut self.tween {
            tween.stop()
        }
    }

    fn choice_labels(&self) -> Array<Gd<DChoice>> {
        self.choice_container()
            .get_children()
            .iter_shared()
            .map(|v| v.cast())
            .collect()
    }

    fn free_choice_labels(&mut self) {
        self.choice_labels()
            .iter_shared()
            .for_each(|mut v| v.queue_free());
    }
}
