//!
//! This module is for pretty much encapsulating all the
//! `midly` stuff in a way that's easy to work with in
//! game-related code.
//!

use std::io::Read;

use godot::engine::file_access::ModeFlags;
use godot::engine::GFile;
use midly::Smf;
use ribbons::unwrap_fmt;

pub struct MidiReader {
    smf: Smf<'static>,
}

impl MidiReader {
    /// Shorthand for `from_godot_path`.
    /// Just pass in the name of the track. No file extension.
    pub fn battle_music(track_name: &str) -> Self {
        let path = format!("res://assets/music/battle/{}.mid", track_name);
        Self::from_godot_path(&path)
    }

    /// Parse a MIDI file from a Godot path
    ///
    /// # Memory Leak
    /// Leaks the data buffer from the file.
    /// Don't run this a gazillion times. It's not expected
    /// to have more than like 200 battle themes in the entire game.
    pub fn from_godot_path(path: &str) -> Self {
        let file = GFile::open(path, ModeFlags::READ);
        let mut file = unwrap_fmt!(file, "Failed to open MIDI file: {}", path);

        let mut data = vec![];
        unwrap_fmt!(file.read(&mut data), "Failed to read MIDI file: {}", path);

        let smf = Smf::parse(data.leak()).expect("Failed to parse MIDI file");
        Self { smf }
    }
}
