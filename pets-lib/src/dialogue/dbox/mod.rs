//!
//! Dialog box class for menus and dialogue text
//!

use godot::classes::{
    AnimationPlayer, Control, HBoxContainer, IPanelContainer, InputEvent,
    PanelContainer, RichTextLabel, Timer,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

mod dchoice;
mod placeholders;
use dchoice::DChoice;

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct DialogBox {
    base: Base<PanelContainer>,

    // state for the current interaction
    active: bool,

    #[init(node = "VBox/Choices/ChoiceAgent")]
    choice_agent: OnReady<Gd<ChoiceAgent>>,

    speaker: String,
    message: String,

    /// Choices to be shown at the next `do_draw` call
    #[var]
    queued_choices: VariantArray,

    #[init(val = DEFAULT_VOX.to_owned())]
    _vox: String, // TODO

    #[init(val = OnReady::manual())]
    text_visibility_timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl DialogBox {
    #[signal]
    fn accept(&self, picked_i: i32);

    #[func]
    fn set_message(&mut self, msg: String) {
        self.message = msg;
    }

    #[func]
    fn set_speaker(&mut self, spk: String) {
        self.speaker = spk;
    }

    // --------------------------------------------------------------

    #[func]
    pub fn singleton() -> Gd<Self> {
        let path = format!("{}/{}", UI_LAYER_NAME, DBOX_NODE_NAME);
        World::singleton().get_node_as::<DialogBox>(path)
    }

    #[func]
    pub fn do_draw(&mut self) {
        // TODO if there are choices to show, show them
        if self.queued_choices.len() > 0 {
            self.recreate_choice_labels();
            self.tween_choices_wave(true);
            self.choice_agent.bind_mut().enable();
            self.queued_choices.clear();
        } else {
            self.free_choice_labels();
        }

        let spk = self
            .active
            .then(|| self.translated_speaker())
            .unwrap_or("".into());

        let msg = self
            .active
            .then(|| self.translated_message())
            .unwrap_or("".into());

        self.spk_txt().set_text(spk);
        self.msg_txt().set_text(msg);

        self.msg_txt().set_visible_characters(0);
        self.text_visibility_timer.start();
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

    /// The current speaker's name, processed.
    /// First processes placeholders, then translation keys.
    fn translated_speaker(&self) -> GString {
        // Unknown => tr("DG_SPK_UNKNOWN"),
        tr(placeholders::process_placeholders(&self.speaker))
    }

    fn translated_message(&self) -> GString {
        let content = self.message.clone();
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
        self.do_draw();
    }

    #[func]
    pub fn open(&mut self) {
        self.open_or_close(true);
    }

    #[func]
    pub fn close(&mut self) {
        self.open_or_close(false);
    }

    /// close the dialog and tween choices away
    pub fn end(&mut self) {
        self.tween_choices_wave(false);
        self.close();
    }

    fn on_confirm_next_page(&mut self) {
        self.base_mut()
            .emit_signal("accept".into(), &[(-1).to_variant()]);
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        // NOTE convention is that the agent is BEFORE the labels
        let picked_i = (choice.get_index() - 1) as u64;
        self.tween_choices_wave(false);

        self.base_mut()
            .emit_signal("accept".into(), &[picked_i.to_variant()]);
    }

    fn _awaiting_choice(&self) -> bool {
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

    fn free_choice_labels(&mut self) {
        let mut cont = self.choice_container();
        let children = children_of_type::<DChoice, _>(cont.clone());

        for mut node in children {
            node.set_name("deleted".into());
            node.queue_free();
            ChoiceAgent::unbind_callables_for(&mut node);
            cont.remove_child(node);
        }
    }

    /// delete old labels and create new default ones
    pub fn recreate_choice_labels(&mut self) {
        self.free_choice_labels();

        let mut cont = self.choice_container();

        for (i, choice) in self.queued_choices.iter_shared().enumerate() {
            godot_print!("Creating choice label: {}", choice);
            let mut dchoice = DChoice::new_container(i, &choice.to_string());

            self.choice_agent
                .bind_mut()
                .bind_callables_for(&mut dchoice);

            cont.add_child(dchoice.clone());

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

            // set timer
            set_timeout(DBOX_CHOICE_WAVE_TIME * i as f64, move || {
                // get the label again using the instance id
                let label = Gd::<DChoice>::try_from_instance_id(label_id);

                if let Ok(label) = label {
                    let mut tw = label.bind().tween_label(up);

                    // if tweening down, delete it after the tween
                    if !up {
                        tw.connect(
                            "finished".into(),
                            label.callable("queue_free"),
                        );
                    }
                };
            });
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
        self.base_mut().add_child(&timer);
        self.text_visibility_timer.init(timer);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.active {
            return;
        }

        let confirming = event.is_action_pressed("ui_accept".into());
        if confirming && !self.is_done_showing_text() {
            self.skip_text_visibility();
            return;
        }

        if confirming {
            mark_input_handled(&self.base());
            self.on_confirm_next_page();
        }
    }
}
