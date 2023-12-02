//!
//! Dialog box class for menus and dialogue text
//!
//! Yes, an important distinction to make is that "dialogue" is
//! the speech between characters, and "dialog" is a window or
//! UI element that displays the dialogue. Don't get too confused!
//!

use dialogical::Speaker::{self, *};
use dialogical::{DialogueEnding, Interaction, Page};
use dialogical::{Metaline, Metaline::*, PageMeta};

use godot::engine::tween::TransitionType;
use godot::engine::{IPanelContainer, InputEvent, PanelContainer, RichTextLabel, Tween, Viewport};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

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
}

#[derive(GodotClass)]
#[class(base=PanelContainer)]
pub struct DialogBox {
    #[base]
    node: Base<PanelContainer>,

    /// the richtextlabel nodes for each current ix choice
    choice_labels: Vec<Gd<RichTextLabel>>,

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

    pub fn set_ix(&mut self, ix: Interaction) {
        self.current_ix = Some(ix);
        self.goto_page(0);
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

        let tw_end = if up {
            // visible y
            viewport_y - node.get_size().y
        } else {
            viewport_y
        };

        let y_tween: Option<Gd<Tween>> = try {
            let mut y_tween = node.create_tween()?;
            y_tween
                .tween_property(
                    node.clone().upcast(),
                    "position:y".into(),
                    Variant::from(tw_end),
                    DBOX_TWEEN_TIME,
                )?
                .from(Variant::from(self.node.get_position().y))?
                .set_trans(DBOX_TWEEN_TRANS)?;
            y_tween
        };

        self.active = up;
        self.tween = y_tween.clone();
        y_tween.unwrap()
    }

    fn free_choice_labels(&mut self) {
        self.choice_labels
            .iter_mut()
            .for_each(|label| label.queue_free());
        self.choice_labels.clear();
    }

    pub fn run_ix_ending(&mut self, ending: &DialogueEnding) {
        use dialogical::Label::*;
        use DialogueEnding::*;

        match ending {
            Choices(choices) => {
                self.free_choice_labels();

                let len = choices.len();
                let width = self.node.get_size().x / len as f32;

                let labels = choices
                    .iter()
                    .enumerate()
                    .map(|(i, choice)| {
                        let mut label = RichTextLabel::new_alloc();
                        label.set_size(Vector2::new(width, DBOX_CHOICE_LABEL_HEIGHT));

                        let name = format!("ChoiceLabel{}", i);
                        label.set_name(name.into());

                        label
                    })
                    .collect::<Vec<_>>();

                let mut timer = godot_tree!().create_timer(0.1).unwrap();
                timer.connect("timeout".into(), self.node.callable("amogus"));

                // uhhhhhh
            }

            Label(Function(_label)) => {
                // TODO run the function
            }

            Label(Goto(_label)) => {
                // TODO "goto" the label
            }

            End => {}
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

            choice_labels: vec![],

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
        if event.is_action_pressed("ui_accept".into()) {
            if !self.active {
                return;
            }

            // go to next page
            let ix = self.current_ix.as_ref().unwrap();
            let pagecount = ix.pages.len();

            self.current_page_number += 1;

            // if end of interaction, close the dialog
            if self.current_page_number >= pagecount {
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
            self.node.get_viewport().unwrap().set_input_as_handled();
        }
    }
}
