#[derive(Debug)]
/// The game's MIDI files have a special code for what each
/// pitch means in terms of in-game beats.
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
