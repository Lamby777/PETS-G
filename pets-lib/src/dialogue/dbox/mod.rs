//!
//! Dialog box class for menus and dialogue text
//!

use dialogical::prelude::*;
use godot::engine::tween::TransitionType;
use godot::engine::{
    HBoxContainer, IPanelContainer, InputEvent, PanelContainer, RichTextLabel,
    Tween,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

mod dchoice;
mod placeholders;
use dchoice::DChoice;
use placeholders::process_placeholders;

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct DialogBox {
    base: Base<PanelContainer>,

    // state for the current interaction
    current_ix: Option<Interaction>,
    current_page_number: usize,
    active: bool,
    awaiting_choice: bool,

    // #[init(default = onready_node(&base, "Choices/ChoiceAgent"))]
    // choices: OnReady<Gd<ChoiceAgent>>,
    #[init(default = MetaPair::from_cloned(Speaker::Narrator))]
    speaker: MetaPair<Speaker>,
    #[init(default = MetaPair::from_cloned(DEFAULT_VOX.to_owned()))]
    vox: MetaPair<String>,

    /// The tween that moves the dialog box on/off screen
    box_tween: Option<Gd<Tween>>,

    /// The tween that makes characters in the message
    /// become visible one by one
    text_tween: Option<Gd<Tween>>,

    // independent from any interaction-related stuff,
    // these are the actual strings that are displayed
    //
    // you can set these directly if you're doing something
    // that's not part of an interaction
    #[init(default = "Cherry".into())]
    spk_txt: GString,
    #[init(default = "[wave amp=50 freq=6]Hello, World![/wave]".into())]
    msg_txt: GString,
}

#[godot_api]
impl DialogBox {
    #[func]
    pub fn do_draw(&mut self) {
        self.goto_current_page();
        self.spk_txt().set_text(self.spk_txt.clone());
        self.msg_txt().set_text(self.msg_txt.clone());
        self.tween_txt_visibility();
    }

    /// Start tweening a text's visible characters from 0% to 100% visible...
    /// See <https://github.com/Lamby777/PETS-G/issues/50>
    pub fn tween_txt_visibility(&mut self) {
        let tw = tween(
            self.msg_txt().upcast(),
            "visible_ratio",
            Some(0.0),
            1.0,
            1.0,
            TransitionType::QUAD,
        );

        // panic if tween failed
        if tw.is_ok() {
            self.text_tween = tw.ok();
        } else {
            panic!("Failed to tween text visibility!");
        }
    }

    /// sets the speaker and message labels to the given page
    pub fn goto_current_page(&mut self) {
        let pageno = self.current_page_number;
        let ix = self.current_ix.as_ref();

        if let Some(ix) = ix {
            let ix = ix.clone();
            let page = ix.pages.get(pageno);
            let page = unwrap_fmt!(page, "Page #{} out of range!", pageno);

            self.update_meta(&page.metadata);
            self.spk_txt = spk_display(&self.speaker.temporary).into();
            self.msg_txt = process_placeholders(&page.content).into();
        } else {
            self.spk_txt = "".into();
            self.msg_txt = "".into();
        };
    }

    /// The method that moves the dialog box (on|off)-screen
    /// and sets the `active` flag
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
        self.box_tween = y_tween.clone().ok();
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
                    ix_id,
                );

                self.set_ix(new_ix.clone());
            }

            Function(fn_id) => {
                let guard = self.base_mut();
                let _ = call_global(fn_id).unwrap();
                drop(guard);
            }
        }
    }

    /// close the dialog and tween choices away
    pub fn end_interaction(&mut self) {
        self.current_page_number = 0;
        self.current_ix = None;
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
    }

    #[func]
    pub fn on_choice_picked(&self, choice: GString) {
        // TODO process input without Wrapped<>
        // Pick(picked_i, _) => {
        //     // we know the ending has to be `Choices` and not a label or end
        //     let ending = self.current_ix_ending().unwrap().clone();
        //     let DialogueEnding::Choices(choices) = ending else {
        //         unreachable!()
        //     };
        //
        //     match &choices[picked_i].label {
        //         // no label means end the interaction
        //         None => self.end_interaction(),
        //
        //         Some(label) => {
        //             let dchoice = &self.choices[picked_i];
        //             let txt = dchoice.bind().txt_label();
        //
        //             tween_choice_to(false, txt);
        //             self.tween_choices_wave(false);
        //
        //             self.run_label(label);
        //         }
        //     }
        //
        //     self.awaiting_choice = false;
        // }
        match choice.to_string().as_str() {
            "Inventory" => todo!(),
            "DebugQuit" => godot_tree().quit(),

            _ => unreachable!(),
        }
    }
}

#[godot_api]
impl IPanelContainer for DialogBox {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed".into(), callable);
        self.choices.bind().set_focus_modes();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let confirming = event.is_action_pressed("ui_accept".into());

        if !self.active {
            return;
        }

        if confirming && let Some(mut tw) = self.text_tween.take() {
            if tw.is_running() {
                // if tweening, skip it and return early
                tw.pause();
                tw.custom_step(1.0);
                tw.kill();
                return;
            }
        }

        if self.awaiting_choice {
            mark_input_handled(&self.base());
            self.process_choice_input();
            return;
        }

        if confirming {
            mark_input_handled(&self.base());
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

    pub fn is_one_page(&self) -> bool {
        let ix = self.current_ix.as_ref().unwrap();
        ix.pages.len() == 1
    }

    pub fn set_ix(&mut self, ix: Interaction) {
        self.current_ix = Some(ix);
        self.current_page_number = 0;
        self.do_draw();

        if self.is_one_page() {
            let ending = self.current_ix_ending().unwrap().clone();
            if let DialogueEnding::Choices(choices) = ending {
                self.recreate_choice_labels(&choices);
                self.tween_choices_wave(true);
                self.awaiting_choice = true;
            }
        }
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
        if let Some(tween) = &mut self.box_tween {
            tween.stop()
        }
    }

    fn free_choice_labels(&mut self) {
        // TODO
        // for node in self.choices.iter_mut() {
        //     node.queue_free();
        // }
    }

    /// delete old labels and create new default ones
    pub fn recreate_choice_labels(&mut self, choices: &[DialogueChoice]) {
        self.free_choice_labels();

        let mut container = self.choice_container();

        let _new_nodes = choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let dchoice = DChoice::new_container(i, &choice.text);
                container.add_child(dchoice.clone().upcast());
                dchoice
            })
            .collect::<Vec<_>>();

        // TODO
        // self.choices.replace_vec(new_nodes);
    }

    pub fn tween_choices_wave(&mut self, up: bool) {
        //     TODO
        //     for (i, cont) in self.choices.iter().enumerate() {
        //         // if moving up, start below the window
        //         if up {
        //             cont.bind()
        //                 .txt_label()
        //                 .set_position(Vector2::new(0.0, DBOX_CHOICE_HEIGHT));
        //         }
        //
        //         // we can't move the label into the closure because of
        //         // thread safety stuff, so just pass in the instance id
        //         let label_id = cont.instance_id();
        //
        //         let func = Callable::from_fn("choice_slide_up", move |_| {
        //             // get the label again using the instance id
        //             let label = Gd::<DChoice>::try_from_instance_id(label_id);
        //
        //             if let Ok(label) = label {
        //                 label.bind().tween_label(up);
        //             };
        //
        //             Ok(Variant::from(()))
        //         });
        //
        //         // set timer
        //         godot_tree()
        //             .create_timer(DBOX_CHOICE_WAVE_TIME * i as f64)
        //             .unwrap()
        //             .connect("timeout".into(), func);
        //     }
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
