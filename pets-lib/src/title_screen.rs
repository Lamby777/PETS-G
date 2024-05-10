//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "Background/MenuChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,
}

#[godot_api]
impl TitleScreen {
    #[func]
    pub fn on_choice_picked(&self, choice: GString) {
        match choice.to_string().as_str() {
            "Play" => {
                // TODO should animate the menu boxes flying
                // off into the right, and the camera goes left

                // bandaid fix of waiting 100ms to avoid changing
                // scene on the same tick as the event cancel.
                // remove this when the intro animation is done
                godot_tree().create_timer(0.1).unwrap().connect(
                    "timeout".into(),
                    Callable::from_fn("", |_| {
                        change_scene!("world");
                        Ok(Variant::nil())
                    }),
                );
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
        self.choices.connect("selection_confirmed".into(), callable);
    }
}
