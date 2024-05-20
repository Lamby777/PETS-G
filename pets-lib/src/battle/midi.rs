//!
//! This module is for pretty much encapsulating all the
//! `midly` stuff in a way that's easy to work with in
//! game-related code.
//!

use crate::prelude::*;
use std::io::Read;

use godot::engine::file_access::ModeFlags;
use godot::engine::GFile;
use midly::Smf;
use nodi::Sheet;
use ribbons::unwrap_fmt;

/// Just pass in the name of the track. No file extension.
///
/// # Memory Leak
/// See [`from_godot_path`] docs for more information.
pub fn battle_music<'a>(track_name: &str) -> Sheet {
    let path = format!("res://assets/music/battle/{}.mid", track_name);
    let Smf { header: _, tracks } = from_godot_path(&path);

    Sheet::sequential(&tracks)
}

/// Parse a MIDI file from a Godot path
///
/// # Memory Leak
/// Leaks the data buffer from the file.
/// Don't run this a gazillion times. It's not expected
/// to have more than like 200 battle themes in the entire game.
fn from_godot_path<'a>(path: &str) -> Smf<'a> {
    let file = GFile::open(path, ModeFlags::READ);
    let mut file = unwrap_fmt!(file, "Failed to open MIDI file: {}", path);

    let mut data = vec![];
    unwrap_fmt!(file.read(&mut data), "Failed to read MIDI file: {}", path);

    Smf::parse(data.leak()).expect("Failed to parse MIDI file")
}
