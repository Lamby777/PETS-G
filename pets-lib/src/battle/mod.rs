//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{INode2D, Node2D, RichTextLabel};
use godot::prelude::*;

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
}

#[allow(unused)]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,

    choices: ChoiceList<BattleState, RichTextLabel>,
    state: BattleState,
}

impl BattleEngine {
    fn menu_confirm(&mut self) {
        // TODO check what the current choice is
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        use BattleState::*;

        match self.state {
            Menu if input.is_action_just_pressed("ui_accept".into()) => {
                self.menu_confirm();
            }

            _ => {
                //
            }
        }
    }
}
