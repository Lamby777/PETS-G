//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!

use crate::consts::choice_lists::*;
use crate::prelude::*;

use godot::engine::control::FocusMode;
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
    focused: Option<Gd<Control>>,

    #[export]
    #[init(val = "position:x".into())]
    tween_property: GString,

    #[export]
    #[init(val = 0.0)]
    tween_normal_value: real,

    #[export]
    #[init(val = 64.0)]
    tween_focused_value: real,

    #[export]
    #[init(val = CHOICE_WAVE_BBCODE.into())]
    bbcode: GString,

    #[export]
    #[var(get, set = set_disabled)]
    disabled: bool,

    #[export]
    #[init(val = ".".into())]
    tween_target_relative: GString,
}

#[godot_api]
impl ChoiceAgent {
    fn _tween_choice(&self, is_picked: bool, node: Gd<Control>) {
        if !node.is_inside_tree() {
            godot_print!("node to tween is not inside tree, returning");
            return;
        }

        let target =
            node.get_node_as::<Node>(self.tween_target_relative.clone());

        // either the node itself is a text label OR check if it
        // has a child to tween
        let txtlabel = target
            .clone()
            .try_cast::<RichTextLabel>()
            .ok()
            .or_else(|| node.try_get_node_as("RichTextLabel"));

        let target_val = match is_picked {
            true => self.tween_focused_value,
            false => self.tween_normal_value,
        };

        // color stuff
        if let Some(label) = txtlabel {
            let target_col = {
                let col = match is_picked {
                    true => "font_selected_color",
                    false => "default_color",
                };

                default_theme().get_color(col.into(), "RichTextLabel".into())
            };

            tween(
                label.clone(),
                "theme_override_colors/default_color",
                None,
                target_col,
                CHOICE_TWEEN_TIME,
                CHOICE_TWEEN_TRANS,
            )
            .unwrap();

            bbcode_toggle(label, CHOICE_WAVE_BBCODE, is_picked);
        }

        if !self.tween_property.is_empty() {
            // tween the custom param
            tween(
                target.clone(),
                self.tween_property.clone(),
                None,
                target_val,
                CHOICE_TWEEN_TIME,
                CHOICE_TWEEN_TRANS,
            )
            .unwrap();
        }
    }

    pub fn parent(&self) -> Gd<Node> {
        self.base()
            .get_parent()
            .expect("choice agent has no parent")
    }

    pub fn choice_labels(&self) -> Vec<Gd<Control>> {
        self.parent()
            .get_children()
            .iter_shared()
            .filter_map(|x| x.try_cast::<Control>().ok())
            .filter(|x| {
                !(x.has_meta("ChoiceAgentIgnore".into())
                    && x.get_meta("ChoiceAgentIgnore".into()).to::<bool>())
            })
            .collect()
    }

    #[func]
    pub fn _tween_choice_on(&mut self, choice: Gd<Control>) {
        self.base_mut()
            .emit_signal("selection_focused".into(), &[choice
                .clone()
                .to_variant()]);
        self.focused = Some(choice.clone());

        self._tween_choice(true, choice);
    }

    #[func]
    pub fn _tween_choice_off(&mut self, choice: Gd<Control>) {
        self.base_mut()
            .emit_signal("selection_unfocused".into(), &[choice
                .clone()
                .to_variant()]);

        if self.focused == Some(choice.clone()) {
            self.focused = None;
        }

        self._tween_choice(false, choice);
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
    pub fn focus_nth(&mut self, n: u32) {
        let mut choices = self.choice_labels();
        let guard = self.base_mut();
        choices[n as usize].call_deferred("grab_focus".into(), &[]);
        drop(guard);
    }

    #[func]
    pub fn set_focus_modes(&self) {
        if !self.base().is_inside_tree() {
            return;
        }

        let mode = match self.disabled {
            true => FocusMode::NONE,
            false => FocusMode::ALL,
        };

        for mut label in self.choice_labels() {
            label.set_focus_mode(mode);
        }
    }

    #[signal]
    fn selection_focused(choice: Gd<Control>) {}

    #[signal]
    fn selection_unfocused(choice: Gd<Control>) {}

    #[signal]
    fn selection_confirmed(choice: Gd<Control>) {}

    pub fn bind_callables_for<N>(&mut self, choice: &mut Gd<N>)
    where
        N: Inherits<Node>,
    {
        Self::unbind_callables_for(choice);
        let choice = &mut choice.clone().upcast();

        let entered = self
            .base()
            .callable("_tween_choice_on")
            .bindv(varray![choice.to_variant()]);
        let exited = self
            .base()
            .callable("_tween_choice_off")
            .bindv(varray![choice.to_variant()]);

        connect_deferred(choice, "focus_entered", entered.clone());
        connect_deferred(choice, "focus_exited", exited.clone());
    }

    pub fn unbind_callables_for<N>(choice: &mut Gd<N>)
    where
        N: Inherits<Node>,
    {
        disconnect_signal(choice, "focus_entered");
        disconnect_signal(choice, "focus_exited");
    }
}

#[godot_api]
impl INode for ChoiceAgent {
    fn ready(&mut self) {
        for node in self.choice_labels() {
            self.bind_callables_for(&mut node.cast::<Control>());
        }

        self.set_focus_modes();
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
            self.focus_nth(0);

            return;
        }

        if is_pressed("ui_accept") {
            // we know it's safe to unwrap here
            let focused = self.focused.clone().unwrap();
            mark_input_handled(&self.base());

            self.base_mut().emit_signal("selection_confirmed".into(), &[
                focused.to_variant(),
            ]);
        }
    }
}
