//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{AnimationPlayer, INode2D, Node2D, RichTextLabel};
use godot::prelude::*;
use num_enum::TryFromPrimitive;

use crate::consts::main_menu::*;
use crate::prelude::*;

mod player;
mod stat_translation;

#[allow(unused)]
mod skills;

#[allow(unused)]
mod rhythm;

#[allow(unused)]
#[derive(Default, PartialEq)]
enum BattleState {
    #[default]
    /// Picking one of the options below
    Menu,

    /// Dodging attacks while clicking to the beat
    Attack,

    /// Selecting a skill to use
    Skill,

    /// Selecting an item to use
    Item,

    /// Run away from the battle
    Run,
}

#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
#[repr(usize)]
enum BattleChoice {
    Attack,
    Skills,
    Items,
    Run,
}

#[allow(unused)]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BattleEngine {
    base: Base<Node2D>,
    state: BattleState,

    choices: Wrapped<(BattleChoice, Gd<RichTextLabel>)>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    animator: OnReady<Gd<AnimationPlayer>>,
}

fn tween_choice_to(is_picked: bool, node: Gd<RichTextLabel>) {
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
        node.upcast(),
        "theme_override_colors/default_color",
        None,
        target_col,
        MENU_TWEEN_TIME,
        MENU_TWEEN_TRANS,
    )
    .unwrap();
}

#[godot_api]
impl BattleEngine {
    #[func]
    pub fn animate_in(&mut self) {
        //
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        let choices = self.base().get_node_as("%BattleChoices");
        self.choices = Wrapped::from_children_of(choices);
    }

    fn process(&mut self, _delta: f64) {
        use crate::wrapped::*;
        let action =
            process_input(&mut self.choices, ListDirection::TopToBottom);

        use ListOperation::*;
        match action {
            Walk(old, (_, new_node)) => {
                if let Some((_, old_node)) = old {
                    tween_choice_to(false, old_node.clone());
                }

                tween_choice_to(true, new_node.clone());
            }

            Pick(_, (choice, _)) => {
                // call different functions depending on the choice
                use BattleChoice::*;
                match choice {
                    Attack => todo!(),
                    Skills => todo!(),
                    Items => todo!(),
                    Run => {
                        // TODO roll, don't always succeed
                        change_scene!("world");
                    }
                }
            }

            Nothing => {}
        }
    }
}
