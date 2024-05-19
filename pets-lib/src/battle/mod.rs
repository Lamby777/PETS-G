//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::object::ConnectFlags;
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

/// How long after a note off event to still consider clicks valid
const LENIENCY_AFTER_BEAT: f64 = 0.02;

/// How long to wait after a click to check if it was early
const LENIENCY_BEFORE_BEAT: f64 = 0.02;

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

    rhythm_state: Option<NoteType>,
    player_clicked: bool,

    #[init(default = OnReady::manual())]
    player_click_timeout: OnReady<Gd<Timer>>,

    #[init(default = onready_node(&base, "BattleMusic"))]
    music: OnReady<Gd<AudioStreamPlayer>>,

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

    /// Called when the player successfully hits a note
    fn on_successful_attack(&mut self) {
        godot_print!("hit");
        self.offset_pos(0, -20);
    }

    fn on_flop_attack(&mut self) {
        godot_print!("flop");
        self.offset_pos(0, 20);
    }

    /// FOR DEBUGGING PURPOSES!!!
    fn offset_pos(&mut self, x: i32, y: i32) {
        let pos = self.base().get_position() + Vector2::new(x as f32, y as f32);
        self.base_mut().set_position(pos);
    }

    #[func]
    pub fn on_note_event(&mut self, on: bool, note: u8) {
        godot_print!("Note event: {} (on: {})", note, on);
        let Some(notetype) = NoteType::from_note(note) else {
            panic!("invalid midi note with code {}", note);
        };

        self.rhythm_state = on.then_some(notetype);

        if on {
            self.on_note_start();
        } else {
            self.offset_pos(-20, 0);

            // if note off received, give X ms of leeway after the
            // ending for them to still hit the note
            let timer = &mut self.rhythm_timer;
            timer.set_wait_time(LENIENCY_AFTER_BEAT);
            timer.start();
            // the timer calls `on_note_end` when it finishes
        }
    }

    #[func]
    pub fn on_early_leniency_expired(&mut self) {
        self.player_clicked = false;
        self.on_flop_attack(); // too early/late
    }

    /// when player tries to attack on a beat
    #[func]
    pub fn on_player_note_hit(&mut self) {
        // if rhythm_state is None, it means the player shouldn't click
        let hit = self.rhythm_state.is_some();

        if hit {
            self.on_successful_attack();
            self.on_note_end();
        } else {
            self.player_clicked = true;

            self.player_click_timeout
                .set_wait_time(LENIENCY_BEFORE_BEAT);
            self.player_click_timeout.start();
        }
    }

    #[func]
    pub fn on_note_start(&mut self) {
        self.offset_pos(20, 0);
        self.rhythm_timer.stop();

        if self.player_clicked {
            // if the player clicked early but `player_clicked` is still
            // true, that means the timer isn't over, so we should count
            // it as close enough to be valid!
            self.player_clicked = false;
            self.player_click_timeout.stop();

            self.on_successful_attack();
        }
    }

    #[func]
    pub fn on_note_end(&mut self) {
        self.rhythm_state = None;
    }

    #[func]
    pub fn intro_over(&mut self) {
        // change state from intro to attack
        self.state = BattleState::Attack { running: false };

        // play the battle music
        self.music.play();
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        self.choices.bind_mut().disable();

        // TODO refactor this dogshit LOL
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed".into(), callable);

        let callable = self.base().callable("on_note_end");
        self.rhythm_timer.connect("timeout".into(), callable);

        // use the rhythm timer as an intro countdown...
        // kinda a hack but whatever
        let mut intro_timer = Timer::new_alloc();
        self.base_mut().add_child(intro_timer.clone().upcast());

        intro_timer.set_wait_time(INTRO_COUNTDOWN_SEC);
        intro_timer.start();

        let callable = self.base().callable("intro_over");
        intro_timer
            .connect_ex("timeout".into(), callable)
            .flags(ConnectFlags::ONE_SHOT.ord() as u32)
            .done();

        let mut click_timer = Timer::new_alloc();
        self.base_mut().add_child(click_timer.clone().upcast());
        let callable = self.base().callable("on_early_leniency_expired");
        click_timer
            .connect_ex("timeout".into(), callable)
            .flags(ConnectFlags::ONE_SHOT.ord() as u32)
            .done();
        self.player_click_timeout.init(click_timer);
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
