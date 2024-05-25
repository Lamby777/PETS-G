//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{AnimationPlayer, Control, PanelContainer};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "%MenuChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    credits_up: bool,
}

#[godot_api]
impl TitleScreen {
    fn credits_panel(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("%CreditsPanel")
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
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
                use crate::consts::dialogue::*;

                let panel = self.credits_panel();
                self.credits_up = !self.credits_up;

                let y = match self.credits_up {
                    true => 0.0,
                    false => 768.0,
                };

                tween(
                    panel,
                    "position:y",
                    None,
                    y,
                    DBOX_TWEEN_TIME,
                    DBOX_TWEEN_TRANS,
                )
                .unwrap();
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
