//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::object::ConnectFlags;
use godot::engine::{
    AnimationPlayer, Control, InputEvent, Texture2D, TextureRect, Timer,
};
use godot::prelude::*;

use crate::consts::battle::*;
use crate::prelude::*;

mod affinities;
mod midi;
mod player;
mod rhythm;
#[allow(unused)]
pub mod skills;
mod stat_translation;

use player::BattleIcon;
use rhythm::BattleMusic;

#[derive(Debug)]
enum AttackFlopReason {
    /// The beat was not clicked
    Skipped,

    /// The player clicked outside of a beat window
    PoorTiming,
}

#[allow(unused)]
#[derive(PartialEq)]
enum MenuSection {
    Main,
    Item,
    Skill,
}

const INTRO_COUNTDOWN_SEC: f64 = 3.0;

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

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BattleEngine {
    base: Base<Node2D>,

    state: BattleState,

    #[init(default = OnReady::manual())]
    battlers: OnReady<Battlers>,
    current_party_member: usize,

    #[init(default = onready_node(&base, "BattleMusic"))]
    music: OnReady<Gd<BattleMusic>>,

    #[init(default = onready_node(&base, "%BattleChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    animator: OnReady<Gd<AnimationPlayer>>,

    #[init(default = onready_node(&base, "BattleIcon"))]
    icon: OnReady<Gd<BattleIcon>>,
}

#[godot_api]
impl BattleEngine {
    /// slowly fade out the black rectangle over the battle scene
    #[func]
    pub fn animate_in(&mut self) {
        self.animator.set_assigned_animation("fade_in".into());
        self.animator.play();
    }

    fn open_dualmenu(&mut self) {
        self.dualmenu_animator()
            .play_animation_forwards("dualmenu_open", true);

        self.state = BattleState::Menu(MenuSection::Main);

        // enable the choice list
        let mut choices = self.choices.bind_mut();
        choices.enable();
        choices.focus_nth(0);
    }

    fn close_dualmenu(&mut self) {
        self.dualmenu_animator()
            .play_animation_forwards("dualmenu_open", false);

        self.state = BattleState::Attack { running: false };

        // disable the choice list
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

    pub fn swap_party_member(&mut self, new_index: usize) {
        self.current_party_member = new_index;
        let id = self.battlers.good_guys[new_index].id();
        godot_print!("Swapped to party member `{}`", id);

        // set battle icon sprite
        self.icon.bind_mut().set_icon(&id);

        // set battle portrait texture
        let mut portrait =
            self.base().get_node_as::<TextureRect>("%PortraitTexture");

        let path = format!("res://assets/textures/portraits/{}.png", id);
        let texture = load::<Texture2D>(path);
        portrait.set_texture(texture);
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Skills" => todo!(),
            "Items" => todo!(),
            "Swap" => {
                let mut next = self.current_party_member + 1;
                if next >= BATTLE_PARTY_SIZE {
                    next = 0;
                }

                self.swap_party_member(next);
            }

            "Run" => {
                // TODO implement running mechanic described earlier
                pcb().bind_mut().battling.clear();

                // TODO don't change scenes, just remove the battle
                // stuff since it's all overlayed on top of the world
                change_scene!("world");
            }

            _ => unreachable!(),
        }
    }

    // ------------------------------------------------------------

    #[func]
    pub fn intro_over(&mut self) {
        // change state from intro to attack
        self.state = BattleState::Attack { running: false };
        self.music.bind_mut().play_battle_music();
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        self.choices.bind_mut().disable();
        self.battlers.init(pcb().bind().new_battlers());

        {
            // intro countdown timer setup
            let mut timer = Timer::new_alloc();
            self.base_mut().add_child(timer.clone().upcast());
            timer.set_wait_time(INTRO_COUNTDOWN_SEC);
            timer.start();
            let callable = self.base().callable("intro_over");
            timer
                .connect_ex("timeout".into(), callable)
                .flags(ConnectFlags::ONE_SHOT.ord() as u32)
                .done();
        }

        connect! {
            self.choices, "selection_confirmed" =>
            self.base(), "on_choice_picked";
        }
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
