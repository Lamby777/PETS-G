//!
//! Data structures related to rhythm in battle
//!
//! For the purposes of battle rhythm...
//! - "Hit" means you clicked at the right time
//! - "Flop" means you click at the wrong time
//! - "Miss" means you didn't click at all
//!

use godot::classes::{
    AudioStreamPlayer, IAudioStreamPlayer, InputEvent, Timer,
};
use godot::prelude::*;

use super::midi::{BattleTrack, MidiReceiver};
use crate::common::*;

/// How long before/after a beat to still consider clicks valid
const LENIENCY_PRE: f64 = 0.1;
const LENIENCY_POST: f64 = 0.04;

#[derive(Clone, Copy, Debug)]
/// The game's MIDI files have a special code for what each
/// pitch means in terms of in-game beats.
///
/// This type explains what kind of note is being played.
pub enum NoteType {
    /// Note that must be hit
    Hit = 60,
}

impl NoteType {
    /// Convert a `u8` from MIDI to its equivalent `NoteType`
    ///
    /// Panics if the note is invalid, for ergonomics.
    /// Use `try_from_note` if you want to handle the error.
    pub fn from_note(note: u8) -> Self {
        unwrap_fmt!(
            Self::try_from_note(note),
            "invalid midi note with code {}",
            note
        )
    }

    pub fn try_from_note(note: u8) -> Option<Self> {
        use NoteType::*;

        Some(match note {
            60 => Hit,
            _ => return None,
        })
    }
}

#[derive(Debug, Default)]
pub struct RhythmState {
    pub player_clicked: bool,
    pub note: Option<NoteType>,
}

impl RhythmState {
    /// Set back to default state
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(GodotClass)]
#[class(init, base=AudioStreamPlayer)]
pub struct BattleMusic {
    base: Base<AudioStreamPlayer>,
    rhythm: RhythmState,

    #[init(val = OnReady::manual())]
    track: OnReady<BattleTrack>,

    /// timer that is in charge of turning `player_clicked` to false
    #[init(val = OnReady::manual())]
    post_click_timer: OnReady<Gd<Timer>>,

    /// timer that gets fired a little bit after the note off event
    #[init(node = "RhythmTimer")]
    note_off_timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl BattleMusic {
    #[signal]
    fn note_hit();

    #[signal]
    fn note_flop();

    #[signal]
    fn note_miss();

    /// Called when the player successfully hits a note
    fn on_attack_hit(&mut self) {
        self.rhythm.reset();
        self.base_mut().emit_signal("note_hit", &[]);
    }

    fn on_attack_flop(&mut self) {
        self.on_attack_flop_or_miss();
        self.base_mut().emit_signal("note_flop", &[]);
    }

    fn on_attack_miss(&mut self) {
        self.on_attack_flop_or_miss();
        self.base_mut().emit_signal("note_miss", &[]);
    }

    fn on_attack_flop_or_miss(&mut self) {
        self.rhythm.player_clicked = false;
    }

    #[func]
    pub fn on_note_on(&mut self, note: u8) {
        self.rhythm.note = Some(NoteType::from_note(note));

        if self.rhythm.player_clicked {
            self.on_attack_hit();
        }

        let timer = &mut self.note_off_timer;
        timer.set_wait_time(LENIENCY_POST);
        timer.start();
    }

    #[func]
    pub fn close_beat(&mut self) {
        // If there was an unclicked note, it's a flop
        if self.rhythm.note.take().is_some() {
            self.on_attack_miss();
        }
    }

    #[func]
    pub fn on_early_leniency_expired(&mut self) {
        // If the player clicked early and there was no note
        // shortly after it, it's a flop
        if self.rhythm.player_clicked {
            self.on_attack_flop();
        }

        self.rhythm.player_clicked = false;
    }

    #[func]
    pub fn on_player_clicked(&mut self) {
        if self.rhythm.player_clicked {
            // prevent the cheat strategy of spamming clicks to always get hits
            // by only allowing one click per note
            self.on_attack_flop();
            return;
        };

        if let Some(_note) = self.rhythm.note.take() {
            // if note is on, it's a hit
            self.on_attack_hit();
        } else {
            // else, set the player click flag on so if a note happens soon,
            // it will count as a hit.
            self.rhythm.player_clicked = true;

            let timer = &mut self.post_click_timer;
            timer.set_wait_time(LENIENCY_PRE);
            timer.start();
        }
    }

    #[func]
    pub fn play_battle_music(&mut self) {
        self.base_mut().play();
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
impl IAudioStreamPlayer for BattleMusic {
    fn ready(&mut self) {
        self.track.init(BattleTrack::new_from_name("alright"));

        // early click timer setup
        let mut timer = Timer::new_alloc();
        timer.set_one_shot(true);
        self.base_mut().add_child(&timer);
        self.post_click_timer.init(timer);

        connect! {
            self.note_off_timer, "timeout" =>
            self.base(), "close_beat";

            self.post_click_timer, "timeout" =>
            self.base(), "on_early_leniency_expired";

            self.track.receiver, "note_on" =>
            self.base(), "on_note_on";

            self.base_mut(), "finished" =>
            self.base(), "play_battle_music";
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            self.on_player_clicked();
        }
    }
}
