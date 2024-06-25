//!
//! Dialog box class for menus and dialogue text
//!

use dialogical::prelude::*;
use godot::engine::global::Side;
use godot::engine::{
    AnimationPlayer, Control, HBoxContainer, IPanelContainer, InputEvent,
    PanelContainer, RichTextLabel, Timer,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

mod dchoice;
mod placeholders;
use dchoice::DChoice;

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

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct DialogBox {
    base: Base<PanelContainer>,

    // state for the current interaction
    current_ix: Option<Interaction>,
    current_page_number: usize,
    active: bool,
    replaces: Vec<(String, String)>,

    #[init(default = onready_node(&base, "VBox/Choices/ChoiceAgent"))]
    choice_agent: OnReady<Gd<ChoiceAgent>>,

    #[init(default = MetaPair::from_cloned(Speaker::Narrator))]
    speaker: MetaPair<Speaker>,
    #[init(default = MetaPair::from_cloned(DEFAULT_VOX.to_owned()))]
    vox: MetaPair<String>,

    #[init(default = OnReady::manual())]
    text_visibility_timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl DialogBox {
    #[func]
    pub fn singleton() -> Gd<Self> {
        let path = format!("{}/{}", UI_LAYER_NAME, DBOX_NODE_NAME);
        World::singleton().get_node_as::<DialogBox>(path)
    }

    #[func]
    pub fn do_draw(&mut self) {
        let ending = self.current_ix_ending().cloned();
        if self.is_on_or_past_last_page()
            && let Some(DialogueEnding::Choices(choices)) = ending
        {
            self.recreate_choice_labels(&choices);
            self.tween_choices_wave(true);
            self.choice_agent.bind_mut().enable();
        }

        self.goto_current_page();
        self.spk_txt().set_text(self.translated_speaker());
        self.msg_txt().set_text(self.translated_message());

        self.msg_txt().set_visible_characters(0);
        self.text_visibility_timer.start();
    }

    #[func]
    pub fn start_ix(&mut self, ix_id: String) {
        self.start_ix_replace(ix_id, vec![]);
    }

    pub fn start_ix_replace(
        &mut self,
        ix_id: String,
        replace: Vec<(String, String)>,
    ) {
        let ix = ix_map().get(&ix_id);
        let ix = unwrap_fmt!(
            ix,
            "Could not find interaction \"{}\" in the interaction map",
            ix_id,
        );

        self.set_ix(ix.clone(), replace);
        self.open();
    }

    /// See <https://github.com/Lamby777/PETS-G/issues/50>
    #[func]
    pub fn text_visibility_tick(&mut self) {
        let mut label = self.msg_txt();
        let visible = label.get_visible_characters();
        label.set_visible_characters(visible + 1);

        // if the next char is whitespace or punctuation, wait longer
        let text = label.get_text().chars().iter().collect::<String>();
        let delay_til_next = match text.chars().nth(visible as usize + 2) {
            Some(next_ch) if next_ch == PAUSE_CHAR => PAUSE_CHAR_DELAY,
            Some(next_ch) if next_ch.is_whitespace() => WHITESPACE_DELAY,
            Some(next_ch) if next_ch.is_ascii_punctuation() => PUNCT_DELAY,

            _ => TEXT_VISIBILITY_DELAY,
        };

        self.text_visibility_timer.set_wait_time(delay_til_next);
        self.text_visibility_timer.start();
    }

    pub fn is_done_showing_text(&self) -> bool {
        let label = self.msg_txt();
        label.get_visible_characters() >= label.get_text().len() as i32
    }

    pub fn skip_text_visibility(&mut self) {
        let mut label = self.msg_txt();
        let len = label.get_text().len() as i32;
        label.set_visible_characters(len);
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
        }
    }

    fn translated_speaker(&self) -> GString {
        if self.current_ix.is_none() {
            return "".into();
        }

        use Speaker::*;

        tr(match &self.speaker.temporary {
            Named(v) => {
                let name = replace_str_all(v, &self.replaces);
                placeholders::process_placeholders(&name).into()
            }

            Narrator => "DG_SPK_NARRATOR".to_owned(),
            Unknown => "DG_SPK_UNKNOWN".to_owned(),
        })
    }

    fn translated_message(&self) -> GString {
        let pageno = self.current_page_number;
        let Some(ix) = self.current_ix.as_ref() else {
            return "".into();
        };

        let page = ix.pages.get(pageno);
        let page = unwrap_fmt!(page, "Page #{} out of range!", pageno);

        let content = page.content.clone();
        let content = replace_str_all(&content, &self.replaces);
        let content = tr(content).to_string();

        placeholders::process_placeholders(&content).into()
    }

    fn anim_player(&self) -> Gd<AnimationPlayer> {
        self.base().get_node_as("AnimationPlayer")
    }

    #[func]
    pub fn open_or_close(&mut self, open: bool) {
        self.active = open;
        self.anim_player().play_animation_forwards("open", open);
    }

    #[func]
    pub fn open(&mut self) {
        self.open_or_close(true);
    }

    #[func]
    pub fn close(&mut self) {
        self.open_or_close(false);
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

                self.set_ix(new_ix.clone(), vec![]);
            }

            Function(fn_id, args) => {
                let guard = self.base_mut();
                let args = args
                    .into_iter()
                    .map(|v| v.to_variant())
                    .collect::<VariantArray>();
                let _ = callv_global(fn_id, args).unwrap();
                drop(guard);
            }

            GDScript(script) => {
                eval(script).unwrap();
            }
        }
    }

    /// close the dialog and tween choices away
    pub fn end_interaction(&mut self) {
        self.current_page_number = 0;
        self.current_ix = None;
        self.tween_choices_wave(false);
        self.close();
    }

    pub fn run_ix_ending(&mut self) {
        use DialogueEnding::*;

        let ending = self.current_ix_ending().unwrap().clone();
        match ending {
            Choices(_) => (), // it's handled in `on_choice_picked`
            Label(label) => self.run_label(&label),
            End => self.end_interaction(),
        }
    }

    fn on_accept(&mut self) {
        // go to next page
        if self.is_on_or_past_last_page() {
            self.run_ix_ending();
        } else {
            self.current_page_number += 1;
        }

        self.do_draw();
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        // NOTE convention is that the agent is BEFORE the labels
        let picked_i = (choice.get_index() - 1) as usize;

        // we know the ending has to be `Choices` and not a label or end
        let ending = self.current_ix_ending().unwrap().clone();
        let DialogueEnding::Choices(choices) = ending else {
            unreachable!()
        };

        match &choices[picked_i].label {
            // no label means end the interaction
            None => self.end_interaction(),

            Some(label) => {
                self.tween_choices_wave(false);
                self.run_label(label);
            }
        }
    }
}

#[godot_api]
impl IPanelContainer for DialogBox {
    fn ready(&mut self) {
        let mut connect = |name: &str, method: &str| {
            let callable = self.base().callable(method);
            // self.choice_agent.connect(name.into(), callable);
            connect_deferred(&mut self.choice_agent, name.into(), callable);
        };

        connect("selection_confirmed", "on_choice_picked");
        connect("selection_focused", "on_choice_focused");
        connect("selection_unfocused", "on_choice_unfocused");

        self.choice_agent.bind_mut().disable();

        let mut timer = Timer::new_alloc();
        timer.set_wait_time(TEXT_VISIBILITY_DELAY);
        timer.connect(
            "timeout".into(),
            self.base().callable("text_visibility_tick"),
        );
        timer.set_one_shot(true);
        self.base_mut().add_child(timer.clone().upcast());
        self.text_visibility_timer.init(timer);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let confirming = event.is_action_pressed("ui_accept".into());

        if !self.active {
            return;
        }

        if confirming && !self.is_done_showing_text() {
            self.skip_text_visibility();
            return;
        }

        if self.awaiting_choice() {
            // TODO NOTE this used to handle stuff but like...
            // i'm not sure if the early return is still necessary
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
    fn awaiting_choice(&self) -> bool {
        !self.choice_agent.bind().get_disabled()
    }

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

    fn set_ix(&mut self, ix: Interaction, replaces: Vec<(String, String)>) {
        self.current_ix = Some(ix);
        self.current_page_number = 0;
        self.replaces = replaces;
        self.do_draw();
    }

    pub fn is_on_or_past_last_page(&self) -> bool {
        let Some(ix) = self.current_ix.as_ref() else {
            return false;
        };

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

    fn free_choice_labels(&mut self) {
        let mut cont = self.choice_container();
        let children = subchildren_of_type::<DChoice, _>(cont.clone());

        for mut node in children {
            node.set_name("deleted".into());
            node.queue_free();
            ChoiceAgent::unbind_callables_for(&mut node);
            cont.remove_child(node.upcast());
        }
    }

    /// delete old labels and create new default ones
    pub fn recreate_choice_labels(&mut self, choices: &[DialogueChoice]) {
        self.free_choice_labels();

        let mut cont = self.choice_container();

        for (i, choice) in choices.iter().enumerate() {
            let mut dchoice = DChoice::new_container(i, &choice.text);

            self.choice_agent
                .bind_mut()
                .bind_callables_for(&mut dchoice);

            cont.add_child(dchoice.clone().upcast());

            // put label below the window
            dchoice.bind().put_label_under();
        }

        self.set_choice_label_focus_directions();
    }

    /// makes it so that all the choice labels refer to each
    /// other in their focus neighbor properties
    pub fn set_choice_label_focus_directions(&self) {
        let nodes = self.choice_nodes();
        let Some(mut previous) = nodes.last().map(Gd::clone) else {
            // if no choices, there's nothing to set
            return;
        };

        // loop should cycle back to start before ending
        let len = nodes.len();
        for mut node in nodes.into_iter().cycle().take(len) {
            let own_path = node.get_path();
            let prev_path = previous.get_path();

            previous.set_focus_next(own_path.clone());
            previous.set_focus_neighbor(Side::RIGHT, own_path);
            node.set_focus_previous(prev_path.clone());
            node.set_focus_neighbor(Side::LEFT, prev_path);

            previous = node;
        }
    }

    pub fn choice_nodes(&self) -> Vec<Gd<DChoice>> {
        self.choice_container()
            .get_children()
            .iter_shared()
            .filter_map(|n| n.try_cast::<DChoice>().ok())
            .collect()
    }

    pub fn tween_choices_wave(&mut self, up: bool) {
        self.choice_agent.bind_mut().set_disabled(!up);
        let choices = self.choice_nodes();

        for (i, cont) in choices.iter().enumerate() {
            // if moving up, start below the window
            if up {
                cont.bind().put_label_under();
            }

            // we can't move the label into the closure because of
            // thread safety stuff, so just pass in the instance id
            let label_id = cont.instance_id();

            let choice_slide_up = move || {
                // get the label again using the instance id
                let label = Gd::<DChoice>::try_from_instance_id(label_id);

                if let Ok(label) = label {
                    label.bind().tween_label(up);
                };
            };

            // set timer
            set_timeout(DBOX_CHOICE_WAVE_TIME * i as f64, choice_slide_up);
        }
    }
}
