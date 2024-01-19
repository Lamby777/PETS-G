//!
//! Dialog box class for menus and dialogue text
//!
//! Yes, an important distinction to make is that "dialogue" is
//! the speech between characters, and "dialog" is a window or
//! UI element that displays the dialogue. Don't get too confused!
//!
//! Update: I've honestly given up trying to remember which is which.
//! Take a look at my branch names and commit messages to see
//! how much I care anymore.
//!

use dialogical::{DialogueChoice, DialogueEnding, Interaction, Label, Metaline, PageMeta, Speaker};

use godot::engine::{
    HBoxContainer, IPanelContainer, InputEvent, PanelContainer, RichTextLabel, Tween,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

mod dchoice;
use dchoice::DChoice;

fn tween_choice_to(is_picked: bool, node: Gd<RichTextLabel>) {
    let target_col = {
        let col = if is_picked {
            "font_selected_color"
        } else {
            "default_color"
        };

        default_theme().get_color(col.into(), "RichTextLabel".into())
    };

    // tween color
    tween(
        node.clone().upcast(),
        "theme_override_colors/default_color",
        None,
        target_col,
        DBOX_CHOICE_TWEEN_TIME,
        DBOX_CHOICE_TWEEN_TRANS,
    )
    .unwrap();

    bbcode_toggle(node, DBOX_SELECTION_BBCODE, is_picked);
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
    awaiting_choice: bool,

    /// the choice label containers
    choices: Wrapped<Gd<DChoice>>,

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
        self.goto_current_page();
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
    }

    /// sets the speaker and message labels to the given page
    pub fn goto_current_page(&mut self) {
        let pageno = self.current_page_number;
        let ix = self.current_ix.as_ref().unwrap().clone();

        let Some(page) = ix.pages.get(pageno) else {
            godot_warn!("Page out of bounds! {}", pageno);
            return;
        };

        self.update_meta(&page.metadata);

        let msg = page.content.clone();
        let spk = spk_display(&self.speaker.temporary);
        self.spk_txt = spk.into();
        self.msg_txt = msg.into();
    }

    /// The method that moves the dialog box (on|off)-screen
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
        self.tween = y_tween.clone().ok();
        y_tween.unwrap()
    }

    pub fn run_label(&mut self, label: &Label) {
        use Label::*;

        match label {
            Goto(ix_id) => {
                let new_ix = ix_map().get(ix_id);
                let new_ix = unwrap_fmt!(
                    new_ix,
                    "GOTO: Could not find interaction with ID: {}",
                    ix_id
                );

                self.set_ix(new_ix.clone());
            }

            Function(_) => {
                todo!("function labels not implemented yet");
            }
        }
    }

    pub fn end_interaction(&mut self) {
        // close the dialog and tween choices away
        self.tween_choices_wave(false);
        self.tween_into_view(false);
    }

    pub fn run_ix_ending(&mut self) {
        use DialogueEnding::*;

        let ending = self.current_ix_ending().unwrap().clone();
        match ending {
            Choices(choices) => {
                self.recreate_choice_labels(&choices);
                self.tween_choices_wave(true);
                self.awaiting_choice = true;
            }

            Label(label) => self.run_label(&label),
            End => self.end_interaction(),
        }
    }

    fn on_accept(&mut self) {
        // go to next page
        self.current_page_number += 1;

        if self.is_on_or_past_last_page() {
            self.run_ix_ending();
        }

        self.do_draw();

        // mark the input as handled
        self.base().get_viewport().unwrap().set_input_as_handled();
    }

    pub fn process_choice_input(&mut self) {
        use crate::wrapped::*;
        let action = process_input(&mut self.choices, ListDir::LeftToRight);

        use ListOperation::*;
        match action {
            Walk(old, new_node) => {
                if let Some(old_node) = old {
                    let old_node = old_node.get_node_as::<RichTextLabel>("Label");
                    tween_choice_to(false, old_node.clone());
                }

                let new_node = new_node.get_node_as::<RichTextLabel>("Label");
                tween_choice_to(true, new_node.clone());
            }

            Pick(picked_i, _) => {
                // we know the ending has to be `Choices` and not a label or end
                let ending = self.current_ix_ending().unwrap().clone();
                let DialogueEnding::Choices(choices) = ending else {
                    unreachable!()
                };

                match &choices[picked_i].label {
                    // no label means end the interaction
                    None => self.end_interaction(),

                    Some(label) => {
                        let dchoice = &self.choices[picked_i];
                        let txt = dchoice.get_node_as("Label");

                        tween_choice_to(false, txt);
                        self.tween_choices_wave(false);

                        self.run_label(label);
                    }
                }

                self.awaiting_choice = false;
            }

            Nothing => {}
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

            choices: Wrapped::default(),
            active: false,
            awaiting_choice: false,
            tween: None,
            current_ix: None,
            current_page_number: 0,
            speaker: MetaPair::from_cloned(Speaker::Narrator),
            vox: MetaPair::from_cloned(DEFAULT_VOX.to_owned()),
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.active {
            return;
        }

        if self.awaiting_choice {
            self.process_choice_input();
            return;
        }

        if event.is_action_pressed("ui_accept".into()) {
            godot_print!(":D");
            self.on_accept();
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
        self.current_page_number = 0;
        self.do_draw();
    }

    pub fn is_on_or_past_last_page(&self) -> bool {
        let ix = self.current_ix.as_ref().unwrap();
        self.current_page_number >= ix.pages.len() - 1
    }

    pub fn current_ix_ending(&self) -> Option<&DialogueEnding> {
        let ix = self.current_ix.as_ref();
        ix.map(|ix| &ix.ending)
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

    fn free_choice_labels(&mut self) {
        for node in self.choices.iter_mut() {
            node.queue_free();
        }
    }

    /// delete old labels and create new default ones
    pub fn recreate_choice_labels(&mut self, choices: &[DialogueChoice]) {
        self.free_choice_labels();

        let mut container = self.choice_container();

        let new_nodes = choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let dchoice = DChoice::new_container(i, &choice.text);
                container.add_child(dchoice.clone().upcast());
                dchoice
            })
            .collect();

        self.choices.replace_vec(new_nodes);
    }

    pub fn tween_choices_wave(&mut self, up: bool) {
        for (i, cont) in self.choices.iter().enumerate() {
            // if moving up, start below the window
            if up {
                cont.get_node_as::<RichTextLabel>("Label")
                    .set_position(Vector2::new(0.0, DBOX_CHOICE_HEIGHT));
            }

            // we can't move the label into the closure because of
            // thread safety stuff, so just pass in the instance id
            let label_id = cont.instance_id();

            let func = Callable::from_fn("choice_slide_up", move |_| {
                // get the label again using the instance id
                let label = Gd::<DChoice>::try_from_instance_id(label_id);

                if let Ok(label) = label {
                    label.bind().tween_label(up);
                };

                Ok(Variant::from(()))
            });

            // set timer
            godot_tree()
                .create_timer(DBOX_CHOICE_WAVE_TIME * i as f64)
                .unwrap()
                .connect("timeout".into(), func);
        }
    }
}

/// Turn a Speaker into a displayable name
///
/// Either the name of the speaker or a special name
/// if it's a narrator or unknown speaker
pub fn spk_display(spk: &Speaker) -> String {
    use Speaker::*;

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
    pub fn set_from<'a>(&mut self, meta: &'a Metaline<T>)
    where
        T: Clone,
    {
        use Metaline::*;
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
