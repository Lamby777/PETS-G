//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::{AnimationPlayer, Control, InputEvent, Timer};
use godot::prelude::*;

use crate::prelude::*;

use self::notes::NoteType;

mod notes;
mod player;
mod stat_translation;

#[allow(unused)]
mod skills;

#[allow(unused)]
#[derive(PartialEq)]
enum MenuSection {
    Main,
    Item,
    Skill,
}

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

    rhythm_state: Option<NoteType>,

    #[init(default = onready_node(&base, "RhythmTimer"))]
    rhythm_timer: OnReady<Gd<Timer>>,

    #[init(default = onready_node(&base, "%BattleChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    animator: OnReady<Gd<AnimationPlayer>>,
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

    #[func]
    pub fn on_note_event(&mut self, on: bool, note: u8) {
        godot_print!("Note hit: {} (on: {})", note, on);
        let Some(notetype) = NoteType::from_note(note) else {
            panic!("invalid midi note with code {}", note);
        };

        self.rhythm_state = on.then_some(notetype);

        if on {
            let timer = &mut self.rhythm_timer;
            timer.set_wait_time(0.5);
            timer.start();
        }
    }

    /// when player tries to attack on a beat
    #[func]
    pub fn on_player_note_hit(&mut self) {
        let hit = self.try_attack();

        if !hit {
            godot_print!("player missed the note, trying again in 20ms");
            let this_id = self.base().instance_id();
            set_timeout(0.02, move || {
                let this = Gd::<Self>::try_from_instance_id(this_id).unwrap();
                this.bind().try_attack();
            });
        }
    }

    /// Returns whether or not it hit, so you can test a
    /// second time later in case it was early
    fn try_attack(&self) -> bool {
        use NoteType::*;

        let Some(state) = &self.rhythm_state else {
            // TODO
            godot_print!("player clicked late/early");
            return false;
        };

        match state {
            Hit => {
                // TODO
                godot_print!("player hit the note");
            }
        }

        true
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed".into(), callable);

        let callable = self.base().callable("note_end");
        self.rhythm_timer.connect("timeout".into(), callable);

        // TODO delay before intro is over
        self.state = BattleState::Attack { running: false };
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            self.toggle_dualmenu();
        } else if event.is_action_pressed("ui_accept".into()) {
            self.on_player_note_hit();
        }
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}
