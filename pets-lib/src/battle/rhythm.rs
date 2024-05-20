//!
//! Data structures related to rhythm in battle
//!

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
