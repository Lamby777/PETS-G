//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!

use crate::consts::choice_lists::*;
use crate::prelude::*;

use godot::engine::{Control, IControl, InputEvent, RichTextLabel};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Choices {
    base: Base<Control>,

    focused: Option<String>,
}

#[godot_api]
impl Choices {
    fn choices(&self) -> Vec<Gd<RichTextLabel>> {
        self.base()
            .get_children()
            .iter_shared()
            .map(|x| x.cast())
            .collect()
    }

    #[func]
    pub fn _tween_choice_on(&mut self, choice: Gd<RichTextLabel>) {
        self.focused = Some(choice.get_text().to_string());
        _tween_choice(true, choice);
    }

    #[func]
    pub fn _tween_choice_off(choice: Gd<RichTextLabel>) {
        _tween_choice(false, choice);
    }

    #[signal]
    fn choice_picked() {}
}

#[godot_api]
impl IControl for Choices {
    // TODO fire signals instead of this, i just copy pasted
    fn ready(&mut self) {
        let mut choices = self.choices();

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
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.focused.is_none() {
            let mut choices = self.choices();
            let guard = self.base_mut();
            choices[0].grab_focus();
            drop(guard);

            mark_input_handled(&self.base());
        }

        let confirming = event.is_action_pressed("ui_accept".into());

        if confirming {
            // TODO signal
            // self.on_choice_picked();
        }
    }
}

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
