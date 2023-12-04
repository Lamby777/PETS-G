//!
//! Dialog box class for menus and dialogue text
//!
//! Yes, an important distinction to make is that "dialogue" is
//! the speech between characters, and "dialog" is a window or
//! UI element that displays the dialogue. Don't get too confused!
//!

use dialogical::Speaker::{self, *};
use dialogical::{DialogueChoice, DialogueEnding, Interaction};
use dialogical::{Metaline, Metaline::*, PageMeta};

use godot::engine::control::SizeFlags;
use godot::engine::{
    HBoxContainer, IPanelContainer, InputEvent, PanelContainer, RichTextLabel, Tween,
};
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

/// slide the label up with a tween
fn tween_choice_label(label: Gd<RichTextLabel>, up: bool) -> Option<Gd<Tween>> {
    let tw_end = if up { DBOX_CHOICE_HEIGHT } else { 0.0 };

    tween(
        label.clone().upcast(),
        "custom_minimum_size:y",
        None,
        tw_end,
        DBOX_TWEEN_TIME,
        DBOX_TWEEN_TRANS,
    )
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
        self.node.get_node_as("VBox/SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as("VBox/Content")
    }

    /// Get the container for choice labels
    fn choice_container(&self) -> Gd<HBoxContainer> {
        self.node.get_node_as("VBox/Choices")
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
        self.speaker.set_from(&meta.speaker);
        self.vox.set_from(&meta.vox);
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

        let y_tween = tween(
            node.clone().upcast(),
            "position:y",
            Some(self.node.get_position().y),
            tw_end,
            DBOX_TWEEN_TIME,
            DBOX_TWEEN_TRANS,
        );

        self.active = up;
        self.tween = y_tween.clone();
        y_tween.unwrap()
    }

    fn choice_labels(&self) -> Array<Gd<RichTextLabel>> {
        self.choice_container()
            .get_children()
            .iter_shared()
            .map(|v| v.cast())
            .collect()
    }

    fn update_choice_labels(&mut self, choices: &[DialogueChoice]) {
        self.choice_labels()
            .iter_shared()
            .for_each(|mut v| v.queue_free());

        let len = choices.len();
        let width = self.node.get_size().x / len as f32;
        let mut container = self.choice_container();

        for (i, choice) in choices.iter().enumerate() {
            let mut label = RichTextLabel::new_alloc();

            let name = format!("Choice{}", i);
            godot_print!("adding choice label {} with text {}", name, choice.text);
            label.set_name(name.into());
            label.set_text(choice.text.clone().into());
            label.set_size(Vector2::new(width, DBOX_CHOICE_HEIGHT));
            label.set_use_bbcode(true);
            label.set_v_size_flags(SizeFlags::SIZE_SHRINK_END);
            label.set_custom_minimum_size(Vector2 { x: 300.0, y: 0.0 });

            container.add_child(label.clone().upcast());

            // queue a timer for the label to slide up
            let delay = DBOX_CHOICE_WAVE_TIME * (i + 1) as f64;
            let mut timer = godot_tree!().create_timer(delay).unwrap();

            // we can't move the label into the closure because of
            // thread safety stuff, so just pass in the instance id
            let label_id = label.instance_id();
            let func = Callable::from_fn("choice_slide_up", move |_| {
                godot_print!("sliding up choice label {}", label_id);

                // get the label again using the instance id
                let label = Gd::from_instance_id(label_id);
                tween_choice_label(label, true)
                    .map(|_| Variant::from(()))
                    .ok_or(())
            });

            timer.connect("timeout".into(), func);
        }
    }

    pub fn run_ix_ending(&mut self, ending: &DialogueEnding) {
        use dialogical::Label::*;
        use DialogueEnding::*;

        match ending {
            Choices(choices) => self.update_choice_labels(choices),

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
            self.node.get_viewport().unwrap().set_input_as_handled();
        }
    }
}
