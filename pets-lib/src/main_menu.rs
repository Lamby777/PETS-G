//!
//! Main menu scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{INode2D, Node2D, RichTextLabel};
use godot::prelude::*;
use num_enum::TryFromPrimitive;

use crate::consts::main_menu::*;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(usize)]
enum MainMenuChoice {
    Play,
    Options,
    Credits,
    Quit,
    DebugBattle,
}

fn tween_choice_to(is_picked: bool, mut node: Gd<RichTextLabel>) {
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

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    #[base]
    node: Base<Node2D>,
    choices: Wrapped<(MainMenuChoice, Gd<RichTextLabel>)>,
}

#[godot_api]
impl INode2D for TitleScreen {
    fn process(&mut self, _delta: f64) {
        use crate::wrapped::*;
        let action = process_input(&mut self.choices, ListDir::TopToBottom);

        use ListOperation::*;
        match action {
            Walk(rev) => {
                // if a node was already selected, tween it back down
                if let Some((_, old_node)) = self.choices.pick() {
                    tween_choice_to(false, old_node.clone());
                }

                self.choices.walk(rev);

                // tween the newly picked node up
                let (_, new_node) = self.choices.pick().unwrap();
                tween_choice_to(true, new_node.clone());
            }

            Pick(_, (choice, _)) => {
                use MainMenuChoice::*;
                match choice {
                    Play => {
                        // TODO should animate the menu boxes flying
                        // off into the right, and the camera goes left
                        change_scene!("world");
                    }

                    Options => {
                        // should scroll right into options menu
                        todo!()
                    }

                    Credits => {
                        // should pull up credits box
                        todo!()
                    }

                    Quit => godot_tree().quit(),

                    DebugBattle => {
                        change_scene!("battle_engine");
                    }
                }
            }

            Nothing => {}
        }
    }

    fn ready(&mut self) {
        // use MainMenuChoice::*;

        // The node that contains the text labels below
        let cont = self.base().get_node_as("Background/MenuChoices");
        self.choices = crate::wrapped::from_children_of(cont);
    }
}
