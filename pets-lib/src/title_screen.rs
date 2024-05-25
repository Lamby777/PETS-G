//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{AnimationPlayer, Control};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "%MenuChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,
}

#[godot_api]
impl TitleScreen {
    #[func]
    pub fn on_choice_picked(&self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Play" => {
                // TODO should animate the menu boxes flying
                // off into the right, and the camera goes left

                let mut anim = self.base().get_node_as::<AnimationPlayer>(
                    "MoveRight/AnimationPlayer",
                );
                anim.set_assigned_animation("main_menu_outro".into());
                anim.play();

                set_timeout(4.0, || {
                    change_scene!("world");
                });
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
}

#[godot_api]
impl INode2D for TitleScreen {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        connect_deferred(&mut self.choices, "selection_confirmed", callable);
    }
}
