//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!

use crate::consts::choice_lists::*;
use crate::prelude::*;

use godot::engine::control::FocusMode;
use godot::engine::object::ConnectFlags;
use godot::engine::{Control, InputEvent, RichTextLabel};
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
    #[init(default = CHOICE_WAVE_BBCODE.into())]
    bbcode: GString,

    #[export]
    #[var(get, set = set_disabled)]
    disabled: bool,
}

#[godot_api]
impl ChoiceAgent {
    pub fn parent(&self) -> Gd<Node> {
        self.base()
            .get_parent()
            .expect("choice agent has no parent")
    }

    pub fn choice_labels(&self) -> Vec<Gd<Control>> {
        // godot_print!(
        //     "getting choice labels for {}",
        //     self.base().get_parent().unwrap().get_name()
        // );

        let choices: Vec<Gd<Control>> = self
            .parent()
            .get_children()
            .iter_shared()
            .filter_map(|x| x.try_cast().ok())
            .collect();

        // godot_print!(
        //     "{:?}",
        //     choices.iter().map(|x| x.get_name()).collect::<Vec<_>>()
        // );

        choices
    }

    #[func]
    pub fn _tween_choice_on(&mut self, choice: Gd<Control>) {
        let name = choice.get_name().to_string();

        self.base_mut()
            .emit_signal("selection_focused".into(), &[name.to_variant()]);
        self.focused = Some(name.clone());

        _tween_choice(true, choice);
    }

    #[func]
    pub fn _tween_choice_off(&mut self, choice: Gd<Control>) {
        let name = choice.get_name().to_string();
        self.base_mut()
            .emit_signal("selection_unfocused".into(), &[name.to_variant()]);

        _tween_choice(false, choice);
    }

    #[func]
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        self.set_focus_modes();
    }

    #[func]
    pub fn enable(&mut self) {
        self.set_disabled(false);
    }

    #[func]
    pub fn disable(&mut self) {
        self.set_disabled(true);
    }

    #[func]
    pub fn focus_first(&mut self) {
        let mut choices = self.choice_labels();
        let guard = self.base_mut();
        choices[0].call_deferred("grab_focus".into(), &[]);
        drop(guard);
    }

    #[func]
    pub fn set_focus_modes(&self) {
        godot_print!("1a");
        let mode = match self.disabled {
            true => FocusMode::NONE,
            false => FocusMode::ALL,
        };

        for mut label in self.choice_labels() {
            godot_print!("1b1");
            label.set_focus_mode(mode);
            godot_print!("1b2");
        }

        godot_print!("1c");
    }

    #[signal]
    fn selection_focused(choice: GString) {}

    #[signal]
    fn selection_unfocused(choice: GString) {}

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

            // choice.connect("focus_entered".into(), entered);
            choice
                .connect_ex("focus_entered".into(), entered)
                .flags(ConnectFlags::DEFERRED.ord() as u32)
                .done();
            choice
                .connect_ex("focus_exited".into(), exited)
                .flags(ConnectFlags::DEFERRED.ord() as u32)
                .done();
        }

        self.set_focus_modes()
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.disabled {
            return;
        }

        let is_pressed = |x: &str| event.is_action_pressed(x.into());

        if self.focused.is_none() {
            if !(is_pressed("ui_up")
                || is_pressed("ui_down")
                || is_pressed("ui_left")
                || is_pressed("ui_right"))
            {
                return; // skip out if it wasn't directional input
            }

            if self.choice_labels().is_empty() {
                return; // if no choices, return
            }

            mark_input_handled(&self.base());

            godot_print!(
                "focusing first choice on {} because nothing is focused",
                self.base().get_parent().unwrap().get_name()
            );
            self.focus_first();

            return;
        }

        if is_pressed("ui_accept")
            && let Some(focused) = &self.focused.clone()
        {
            mark_input_handled(&self.base());

            self.base_mut().emit_signal("selection_confirmed".into(), &[
                focused.to_variant(),
            ]);
        }
    }
}

// TODO vertical tweening
fn _tween_choice(is_picked: bool, node: Gd<Control>) {
    let on_off = if is_picked { "on" } else { "off" };
    godot_print!("tweening {} {}", node.get_name(), on_off);

    let node = node.try_cast::<RichTextLabel>().unwrap_or_else(|node| {
        godot_print!("getting text inside container");
        node.get_node_as("RichTextLabel")
    });

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

    bbcode_toggle(node, CHOICE_WAVE_BBCODE, is_picked);
}
