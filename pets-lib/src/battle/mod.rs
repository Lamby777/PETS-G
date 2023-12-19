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
        let input = Input::singleton();

        // TODO up/down

        if !input.is_action_just_pressed("ui_accept".into()) {
            // do nothing
            return;
        }

        // TODO check what the current choice is
        // and change the state accordingly
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn process(&mut self, _delta: f64) {
        use BattleState::*;

        (match self.state {
            Menu => Self::menu_confirm,
            Attack => todo!(),
            Skill => todo!(),
            Item => todo!(),
        })(self);
    }
}
