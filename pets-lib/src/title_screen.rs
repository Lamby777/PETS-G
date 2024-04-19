//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{Control, INode2D, InputEvent, Node2D, RichTextLabel};
use godot::prelude::*;

use crate::consts::title_screen::*;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    focused: Option<String>,
}

#[godot_api]
impl TitleScreen {
    pub fn choices(&self) -> Vec<Gd<RichTextLabel>> {
        self.base()
            .get_node_as::<Control>("Background/MenuChoices")
            .get_children()
            .iter_shared()
            .map(|x| x.cast())
            .collect()
    }

    pub fn on_choice_picked(&self) {
        let Some(focused) = &self.focused else {
            return;
        };

        match focused.as_str() {
            "Play" => {
                // TODO should animate the menu boxes flying
                // off into the right, and the camera goes left
                change_scene!("world");
            }

            "Options" => {
                // should scroll right into options menu
                todo!()
            }

            "Credits" => {
                // should pull up credits box
                todo!()
            }

            "Quit" => godot_tree().quit(),

            _ => unreachable!(),
        }
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
}

#[godot_api]
impl INode2D for TitleScreen {
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

        choices[0].grab_focus();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let confirming = event.is_action_pressed("ui_accept".into());
        if !confirming {
            return;
        }

        if let Some(focused) = &self.focused {
            godot_print!("Picked: {}", focused);
            // self.on_choice_picked(focused);
        } else {
            godot_print!("No choice focused");
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
        MENU_TWEEN_TIME,
        MENU_TWEEN_TRANS,
    )
    .unwrap();

    // tween color
    tween(
        node.clone().upcast(),
        "theme_override_colors/default_color",
        None,
        target_col,
        MENU_TWEEN_TIME,
        MENU_TWEEN_TRANS,
    )
    .unwrap();

    // make it wavy (or not) :3
    bbcode_toggle(node, MENU_WAVE_BBCODE, is_picked);
}
