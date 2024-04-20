//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!

use crate::consts::choice_lists::*;
use crate::prelude::*;

use godot::engine::control::FocusMode;
use godot::engine::{INode, InputEvent, Node, RichTextLabel};
use godot::prelude::*;

/// This class should be placed under any control that has
/// child RichTextLabels that represent choices. It will
/// handle all the tweening and input for you.
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ChoiceAgent {
    base: Base<Node>,

    /// Name of the currently focused choice
    focused: Option<String>,

    #[export]
    disabled: bool,
}

#[godot_api]
impl ChoiceAgent {
    pub fn choice_labels(&self) -> Vec<Gd<RichTextLabel>> {
        self.base()
            .get_parent()
            .expect("choice agent has no parent")
            .get_children()
            .iter_shared()
            .filter_map(|x| x.try_cast().ok())
            .collect()
    }

    #[func]
    pub fn _tween_choice_on(&mut self, choice: Gd<RichTextLabel>) {
        let newly_focused = choice.get_text().to_string();

        self.base_mut().emit_signal("selection_changed".into(), &[
            newly_focused.to_variant(),
        ]);
        self.focused = Some(newly_focused);

        _tween_choice(true, choice);
    }

    #[func]
    pub fn set_focus_modes(&self) {
        let mode = match self.disabled {
            true => FocusMode::NONE,
            false => FocusMode::ALL,
        };

        self.choice_labels()
            .iter_mut()
            .for_each(|x| x.set_focus_mode(mode));
    }

    #[func]
    pub fn _tween_choice_off(choice: Gd<RichTextLabel>) {
        _tween_choice(false, choice);
    }

    #[signal]
    fn selection_changed(choice: GString) {}

    #[signal]
    fn selection_confirmed(choice: GString) {}
}

#[godot_api]
impl INode for ChoiceAgent {
    fn ready(&mut self) {
        let mut choices = self.choice_labels();

        for choice in &mut choices {
            let entered = self
                .base()
                .callable("_tween_choice_on")
                .bindv(varray![choice.to_variant()]);

            let exited = self
                .base()
                .callable("_tween_choice_off")
                .bindv(varray![choice.to_variant()]);

            choice.connect("focus_entered".into(), entered);
            choice.connect("focus_exited".into(), exited);
        }

        self.set_focus_modes()
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.disabled {
            return;
        }

        if self.focused.is_none() {
            let mut choices = self.choice_labels();
            let guard = self.base_mut();
            choices[0].grab_focus();
            drop(guard);

            mark_input_handled(&self.base());
        }

        let confirming = event.is_action_pressed("ui_accept".into());
        if confirming && let Some(focused) = &self.focused.clone() {
            self.base_mut().emit_signal("selection_confirmed".into(), &[
                focused.to_variant(),
            ]);
        }
    }
}

// TODO vertical tweening
fn _tween_choice(is_picked: bool, node: Gd<RichTextLabel>) {
    let target_x = if is_picked { 64.0 } else { 0.0 };

    let target_col = {
        let col = if is_picked {
            "font_selected_color"
        } else {
            "default_color"
        };

        default_theme().get_color(col.into(), "RichTextLabel".into())
    };

    // tween x
    tween(
        node.clone().upcast(),
        "position:x",
        None,
        target_x,
        CHOICE_TWEEN_TIME,
        CHOICE_TWEEN_TRANS,
    )
    .unwrap();

    // tween color
    tween(
        node.clone().upcast(),
        "theme_override_colors/default_color",
        None,
        target_col,
        CHOICE_TWEEN_TIME,
        CHOICE_TWEEN_TRANS,
    )
    .unwrap();

    // make it wavy (or not) :3
    bbcode_toggle(node, CHOICE_WAVE_BBCODE, is_picked);
}
