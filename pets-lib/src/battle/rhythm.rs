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
    Hit,
}

impl NoteType {
    pub fn from_note(note: u8) -> Option<Self> {
        Some(match note {
            60 => Self::Hit,
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
