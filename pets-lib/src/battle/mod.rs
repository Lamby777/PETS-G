//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{INode2D, InputEvent, Node2D, RichTextLabel};
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

    choices: ChoiceList<Gd<RichTextLabel>>,
    state: BattleState,
}

#[godot_api]
impl INode2D for BattleEngine {
    fn input(&mut self, event: Gd<InputEvent>) {
        use BattleState::*;

        match self.state {
            Menu if event.is_action_pressed("ui_accept".into()) => {}
            _ => {}
        }
    }
}
