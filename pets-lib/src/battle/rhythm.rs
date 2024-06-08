//!
//! Data structures related to rhythm in battle
//!

use godot::prelude::*;

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
        ribbons::unwrap_fmt!(
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
}

#[godot_api]
impl BattleMusic {}

#[godot_api]
impl IAudioStreamPlayer for BattleMusic {}
