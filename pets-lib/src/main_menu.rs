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

    // set bbcode
    // extremely ugly and hacky solution, but...
    // how else could you work with in-band formatting? :P
    let old_text = node.get_text();
    let new_text = if is_picked {
        // prepend [wave] stuff to msg
        format!("{}{}", MENU_WAVE_BBCODE, old_text)
    } else {
        // slice off [wave] stuff from start
        let st: String = old_text.into();
        st[MENU_WAVE_BBCODE.len()..].to_owned()
    };

    node.set_text(new_text.into());
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
        process_input(&mut self.choices, ListDir::TopToBottom);
    }

    fn ready(&mut self) {
        // use MainMenuChoice::*;

        // The node that contains the text labels below
        let cont = self.base().get_node_as("Background/MenuChoices");

        use crate::wrapped::from_children_of;
        self.choices = from_children_of(cont);
        //     Some(|old, (_, new)| {
        //         tween_choice_to(true, new.clone());
        //         if let Some((_, old)) = old {
        //             tween_choice_to(false, old.clone());
        //         }
        //     }),
        //     Some(|_, (choice, _)| {
        //         match choice {
        //             Play => {
        //                 // TODO should animate the menu boxes flying
        //                 // off into the right, and the camera goes left
        //                 change_scene!("world");
        //             }
        //
        //             Options => {
        //                 // should scroll right into options menu
        //                 todo!()
        //             }
        //
        //             Credits => {
        //                 // should pull up credits box
        //                 todo!()
        //             }
        //
        //             Quit => godot_tree().quit(),
        //
        //             DebugBattle => {
        //                 change_scene!("battle_engine");
        //             }
        //         }
        //     }),
        // );
    }
}
