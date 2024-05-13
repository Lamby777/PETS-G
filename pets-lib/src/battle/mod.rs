//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{AnimationPlayer, Control, InputEvent};
use godot::prelude::*;

use crate::prelude::*;

mod player;
mod stat_translation;

#[allow(unused)]
mod skills;

#[allow(unused)]
mod rhythm;

#[allow(unused)]
#[derive(PartialEq)]
enum MenuSection {
    Main,
    Item,
    Skill,
}

#[allow(unused)]
#[derive(Default, PartialEq)]
enum BattleState {
    /// Few seconds countdown before the music starts
    #[default]
    Intro,

    /// Dodging attacks while clicking to the beat
    Attack {
        /// Running away from battle?
        ///
        /// While running away, you won't be able
        /// to fight back, but once the countdown
        /// is over, you'll escape the fight.
        running: bool,
    },

    /// Has the menu open
    Menu(MenuSection),
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
    /// slide the battle screen into view
    #[func]
    pub fn animate_in(&mut self) {
        self.animator.set_assigned_animation("fade_in".into());
        self.animator.play();
    }

    fn open_dualmenu(&mut self) {
        let mut anim = self.dualmenu_animator();
        anim.set_assigned_animation("dualmenu_open".into());
        anim.play();

        self.state = BattleState::Menu(MenuSection::Main);

        // enable the choice list
        let mut choices = self.choices.bind_mut();
        choices.enable();
        choices.focus_nth(0);
    }

    fn close_dualmenu(&mut self) {
        let mut anim = self.dualmenu_animator();
        anim.set_assigned_animation("dualmenu_open".into());
        anim.play_backwards();

        self.state = BattleState::Attack { running: false };

        // enable the choice list
        self.choices.bind_mut().disable();
    }

    fn toggle_dualmenu(&mut self) {
        use BattleState::*;

        match self.state {
            // no menu while running away or in intro
            Intro | Attack { running: true } => return,

            // close menu if open
            Menu(_) => self.close_dualmenu(),

            // open menu if closed
            // (exhaustive match in case we add more states later)
            Attack { running: false } => self.open_dualmenu(),
        }
    }

    pub fn dualmenu(&self) -> Gd<Control> {
        self.base().get_node_as::<Control>("Menu/DualMenu")
    }

    pub fn dualmenu_animator(&self) -> Gd<AnimationPlayer> {
        self.dualmenu()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
    }

    #[func]
    pub fn on_choice_picked(&self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Skills" => todo!(),
            "Items" => todo!(),
            "Swap" => todo!(),

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

        // TODO delay before intro is over
        self.state = BattleState::Attack { running: false };
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            self.toggle_dualmenu();
        }
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}
