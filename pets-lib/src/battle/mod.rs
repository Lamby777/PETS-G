//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{Control, INode2D, Node2D, RichTextLabel};
use godot::prelude::*;

use crate::consts::main_menu::*;
use crate::prelude::*;

mod player;
mod stat_translation;

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

#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
enum BattleChoice {
    Attack,
    Skill,
    Item,
    Run,
}

#[allow(unused)]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,

    choices: ChoiceList<BattleChoice, RichTextLabel>,
    state: BattleState,
}

fn tween_choice_to(is_picked: bool, node: Gd<RichTextLabel>) {
    let target_x = if is_picked { 64.0 } else { 0.0 };

    let target_col = {
        let col = if is_picked {
            "font_selected_color"
        } else {
            "default_color"
        };

        default_theme!().get_color(col.into(), "RichTextLabel".into())
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
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        use BattleChoice::*;

        // The node that contains the text labels below
        let cont = self.node.get_node_as::<Control>("%Choices");
        let nodes_map = [
            // all the main menu label you can pick
            (Attack, "Attack"),
            (Skill, "Skills"),
            (Item, "Items"),
            (Run, "Run"),
        ]
        .into_iter()
        .map(|(e, nodename)| (e, cont.get_node_as(nodename)))
        .collect::<Vec<_>>();

        self.choices = ChoiceList::new(nodes_map, tween_choice_to, |choice| {
            // call different functions depending on the choice
            match choice {
                Attack => todo!(),
                Skill => todo!(),
                Item => todo!(),
                Run => todo!(),
            }
        });
    }

    fn process(&mut self, _delta: f64) {
        self.choices.process_input();
    }
}
