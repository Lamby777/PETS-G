//!
//! Title screen scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::classes::tween::TransitionType;
use godot::classes::{
    AnimationPlayer, ColorRect, Control, InputEvent, PanelContainer,
};
use godot::prelude::*;

use crate::common::*;

const CREDITS_TWEEN_TIME: f64 = 0.5;
const BLACK_FADE_TIME: f64 = 1.0;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct TitleScreen {
    base: Base<Node2D>,

    #[init(node = "%MenuChoices/ChoiceAgent")]
    choices: OnReady<Gd<ChoiceAgent>>,

    credits_up: bool,

    #[init(node = "%SaveFilesContainer")]
    save_button_cont: OnReady<Gd<Control>>,

    #[init(node = "%SaveFilesContainer/ChoiceAgent")]
    save_choices: OnReady<Gd<ChoiceAgent>>,

    #[init(node = "%BlackFade")]
    black: OnReady<Gd<ColorRect>>,

    #[init(node = "%CreditsPanel")]
    credits_panel: OnReady<Gd<PanelContainer>>,
}

#[godot_api]
impl TitleScreen {
    fn anim_out(&self) {
        fade_black(&self.black, true, BLACK_FADE_TIME);

        let mut anim = self
            .base()
            .get_node_as::<AnimationPlayer>("MoveRight/AnimationPlayer");
        anim.set_assigned_animation("main_menu_outro");
        anim.play();
    }

    fn start_game(&self) {
        self.anim_out();

        set_timeout(1.5, || {
            change_scene!("world");
        });
    }

    #[func]
    fn on_save_file_picked(&mut self, choice: Gd<Control>) {
        let slot = choice.get_name().to_string().parse::<usize>().unwrap();

        let save = SaveFile::load_from(slot as u8).unwrap();
        si().bind_mut().load_save_state(save);

        self.start_game();
    }

    fn show_save_files(&mut self, show: bool) {
        let mut anim = self
            .save_button_cont
            .get_node_as::<AnimationPlayer>("../AnimationPlayer");

        anim.play_animation_forwards("open", show);
        self.save_choices.bind_mut().set_disabled(!show);
        self.choices.bind_mut().set_disabled(show);
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
                let panel = &mut self.credits_panel;
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

    fn save_files_shown(&self) -> bool {
        !self.save_choices.bind().get_disabled()
    }

    // fn save_buttons(&self) -> Vec<Gd<PanelContainer>> {
    //     self.save_button_cont
    //         .get_children()
    //         .iter_shared()
    //         .skip(1) // skip the partially-shown decoration one
    //         .map(Gd::cast)
    //         .collect()
    // }
}

#[godot_api]
impl INode2D for TitleScreen {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        connect_deferred(&mut self.choices, "selection_confirmed", callable);

        let callable = self.base().callable("on_save_file_picked");
        self.save_choices.connect("selection_confirmed", &callable);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_cancel") && self.save_files_shown() {
            self.show_save_files(false);
        }
    }
}
