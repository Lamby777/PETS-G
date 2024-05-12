//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{AnimationPlayer, Control};
use godot::prelude::*;

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

#[allow(unused)]
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BattleEngine {
    base: Base<Node2D>,
    state: BattleState,

    #[init(default = onready_node(&base, "%BattleChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    animator: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl BattleEngine {
    #[func]
    pub fn animate_in(&mut self) {
        //
    }

    #[func]
    pub fn on_choice_picked(&self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Attack" => todo!(),
            "Skills" => todo!(),
            "Items" => todo!(),

            "Run" => {
                // TODO roll, don't always succeed
                PlayerCB::singleton().bind_mut().in_battle = false;

                // TODO don't change scenes, just remove the battle
                // stuff since it's all overlayed on top of the world
                change_scene!("world");
            }

            _ => unreachable!(),
        }
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed".into(), callable);
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}
