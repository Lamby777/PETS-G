//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::object::ConnectFlags;
use godot::engine::{AnimationPlayer, Control, InputEvent, Timer};
use godot::prelude::*;

use crate::prelude::*;

mod midi;
mod player;
mod rhythm;
#[allow(unused)]
mod skills;
mod stat_translation;

use midi::{BattleTrack, MidiReceiver};
use rhythm::*;

#[allow(unused)]
#[derive(PartialEq)]
enum MenuSection {
    Main,
    Item,
    Skill,
}

const INTRO_COUNTDOWN_SEC: f64 = 3.0;

/// How long before/after a beat to still consider clicks valid
const LENIENCY_PRE: f64 = 0.08;
const LENIENCY_POST: f64 = 0.02;

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

    rhythm: RhythmState,

    #[init(default = OnReady::manual())]
    track: OnReady<BattleTrack>,

    /// timer that is in charge of turning `player_clicked` to false
    #[init(default = OnReady::manual())]
    post_click_timer: OnReady<Gd<Timer>>,

    /// timer that gets fired a little bit after the note off event
    #[init(default = onready_node(&base, "RhythmTimer"))]
    note_off_timer: OnReady<Gd<Timer>>,

    #[init(default = onready_node(&base, "BattleMusic"))]
    music: OnReady<Gd<AudioStreamPlayer>>,

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
                // TODO implement running mechanic described earlier
                PlayerCB::singleton().bind_mut().in_battle = false;

                // TODO don't change scenes, just remove the battle
                // stuff since it's all overlayed on top of the world
                change_scene!("world");
            }

            _ => unreachable!(),
        }
    }

    // ------------------------------------------------------------

    /// FOR DEBUGGING PURPOSES!!!
    fn offset_pos(&mut self, x: i32, y: i32) {
        let pos = self.base().get_position() + Vector2::new(x as f32, y as f32);
        self.base_mut().set_position(pos);
    }

    /// Called when the player successfully hits a note
    fn on_successful_attack(&mut self) {
        self.offset_pos(0, -20);

        self.rhythm.reset();
    }

    fn on_flop_attack(&mut self) {
        self.offset_pos(0, 20);

        self.rhythm.player_clicked = false;
    }

    // ------------------------------------------------------------

    #[func]
    pub fn on_note_on(&mut self, note: u8) {
        self.rhythm.note = Some(NoteType::from_note(note));

        if self.rhythm.player_clicked {
            self.on_successful_attack();
        }

        self.base()
            .get_node_as::<AudioStreamPlayer>("ClickSFX")
            .play();

        let timer = &mut self.note_off_timer;
        timer.set_wait_time(LENIENCY_POST);
        timer.start();
    }

    #[func]
    pub fn close_beat(&mut self) {
        self.rhythm.note = None;
    }

    #[func]
    pub fn on_early_leniency_expired(&mut self) {
        if self.rhythm.player_clicked {
            self.on_flop_attack();
        }

        self.rhythm.player_clicked = false;
    }

    #[func]
    pub fn on_player_clicked(&mut self) {
        if self.rhythm.player_clicked {
            return;
        };

        if let Some(_note) = self.rhythm.note.take() {
            // if note is on, it's a hit
            self.on_successful_attack();
        } else {
            // else, set the player click flag on so if a note happens soon,
            // it will count as a hit.
            self.rhythm.player_clicked = true;

            let timer = &mut self.post_click_timer;
            timer.set_wait_time(LENIENCY_PRE);
            timer.start();
        }
    }

    // ------------------------------------------------------------

    #[func]
    pub fn intro_over(&mut self) {
        // change state from intro to attack
        self.state = BattleState::Attack { running: false };

        // play the battle music
        self.music.play();

        let iid = self.track.receiver.instance_id();
        let sheet = self.track.sheet.clone();
        let ticker = self.track.ticker.clone();

        thread::spawn(move || {
            let receiver = GdW(Gd::<MidiReceiver>::from_instance_id(iid));
            let mut player = nodi::Player::new(ticker, receiver);

            player.play(&sheet);
        });
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        self.choices.bind_mut().disable();
        self.track.init(BattleTrack::new_from_name("alright"));

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

        // early click timer setup
        let mut timer = Timer::new_alloc();
        timer.set_one_shot(true);
        self.base_mut().add_child(timer.clone().upcast());
        self.post_click_timer.init(timer);

        connect! {
            self.choices, "selection_confirmed" =>
            self.base(), "on_choice_picked";

            self.note_off_timer, "timeout" =>
            self.base(), "close_beat";

            self.post_click_timer, "timeout" =>
            self.base(), "on_early_leniency_expired";

            self.track.receiver, "note_on" =>
            self.base(), "on_note_on";
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            self.toggle_dualmenu();
        } else if event.is_action_pressed("ui_accept".into()) {
            self.on_player_clicked();
        }
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}
