//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::tween::TransitionType;
use godot::engine::{
    AnimationPlayer, ColorRect, Control, HBoxContainer, PanelContainer,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::prelude::*;

const CREDITS_TWEEN_TIME: f64 = 0.5;
const BLACK_FADE_TIME: f64 = 1.0;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "%MenuChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    credits_up: bool,

    #[init(default = onready_node(&base, "%SaveFilesContainer"))]
    save_button_cont: OnReady<Gd<HBoxContainer>>,
}

#[godot_api]
impl TitleScreen {
    fn credits_panel(&self) -> Gd<PanelContainer> {
        self.base().get_node_as("%CreditsPanel")
    }

    fn black(&self) -> Gd<ColorRect> {
        self.base().get_node_as("%BlackFade")
    }

    fn anim_out(&self) {
        fade_black(self.black(), true, BLACK_FADE_TIME);

        let mut anim = self
            .base()
            .get_node_as::<AnimationPlayer>("MoveRight/AnimationPlayer");
        anim.set_assigned_animation("main_menu_outro".into());
        anim.play();
    }

    fn start_game(&self) {
        self.anim_out();

        set_timeout(1.5, || {
            change_scene!("world");
        });
    }

    #[func]
    fn on_save_file_picked(&mut self, slot: u64) {
        godot_print!("{}", slot);

        let save = SaveFile::load_from(slot as u8).unwrap();
        si().bind_mut().load_save_state(&save);

        self.start_game();
    }

    fn show_save_files(&mut self, show: bool) {
        let mut anim = self
            .save_button_cont
            .get_node_as::<AnimationPlayer>("../AnimationPlayer");

        anim.set_assigned_animation("open".into());
        match show {
            true => anim.play(),
            false => anim.play_backwards(),
        }
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Play" => {
                self.show_save_files(true);
            }

            "Options" => {
                // should scroll right into options menu
                todo!()
            }

            "Credits" => {
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
                    CREDITS_TWEEN_TIME,
                    TransitionType::QUAD,
                )
                .unwrap();
            }

            "Quit" => {
                self.anim_out();

                set_timeout(1.1, || {
                    godot_tree().quit();
                });
            }

            _ => unreachable!(),
        }
    }

    fn save_buttons(&self) -> Vec<Gd<PanelContainer>> {
        self.save_button_cont
            .get_children()
            .iter_shared()
            .skip(1) // skip the partially-shown decoration one
            .map(Gd::cast)
            .collect()
    }
}

#[godot_api]
impl INode2D for TitleScreen {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        connect_deferred(&mut self.choices, "selection_confirmed", callable);

        let _callable = self.base().callable("on_save_file_picked");

        // for (i, mut v) in self.save_buttons().into_iter().enumerate() {
        //     let callable = callable.bindv(varray![i as u64 + 1]);
        //     v.connect("".into(), callable);
        // }
    }
}
