//!
//! Main menu scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{Control, Node2D, Node2DVirtual, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;

use field_count::FieldCount;

#[derive(FieldCount)]
struct Choices {
    play: Gd<RichTextLabel>,
    options: Gd<RichTextLabel>,
    credits: Gd<RichTextLabel>,
    quit: Gd<RichTextLabel>,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TitleScreen {
    #[base]
    node: Base<Node2D>,
    si: Gd<StatsInterface>,

    // Option because not init til _ready()
    choices: Option<Choices>,

    // null if game just started (no choice hovered)
    current_choice: Option<u8>,
}

impl TitleScreen {
    fn change_menu_choice(&mut self, diff: i16) {
        let old_choice = self.current_choice;

        let res = if let Some(n) = old_choice {
            let n = n as i16;
            let total_choices = Choices::field_count() as i16;
            (n + diff).rem_euclid(total_choices)
        } else {
            0
        };

        self.current_choice = Some(res as u8);
    }
}

#[godot_api]
impl Node2DVirtual for TitleScreen {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),

            choices: None,
            current_choice: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_action_just_pressed("ui_down".into()) {
            self.change_menu_choice(1);
        } else if input.is_action_just_pressed("ui_up".into()) {
            self.change_menu_choice(-1);
        }
    }

    fn ready(&mut self) {
        // The node that contains the text labels for
        // all the main menu options you can pick
        let cont = self.node.get_node_as::<Control>("Background/MenuChoices");

        let choices = Choices {
            play: cont.get_node_as("Play"),
            options: cont.get_node_as("Options"),
            credits: cont.get_node_as("Credits"),
            quit: cont.get_node_as("Quit"),
        };

        self.choices = Some(choices);
    }
}
