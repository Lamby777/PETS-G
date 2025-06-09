//!
//! This module is for pretty much encapsulating all the
//! `midly` stuff in a way that's easy to work with in
//! game-related code.
//!

use crate::common::*;

use std::io::Read;

use godot::classes::file_access::ModeFlags;
use godot::prelude::*;
use godot::tools::GFile;

use midly::Smf;
use nodi::timers::Ticker;
use nodi::{Connection, MidiEvent, Sheet};

/// Receives MIDI events and handles them for P/E/T/S
///
/// Acts like a MIDI player but instead of playing stuff,
/// it sends note events to the battle engine.
///
/// It is implemented as a Godot `Node` so that it can send
/// signals n stuff, and be a child of the `BattleEngine`.
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MidiReceiver {
    base: Base<Node>,
}

impl Connection for GdW<MidiReceiver> {
    fn play(&mut self, event: MidiEvent) -> bool {
        use midly::MidiMessage::*;

        match event.message {
            NoteOn { key, vel: _ } => {
                self.bind_mut().on_note_event(true, key.into());
            }

            NoteOff { key, vel: _ } => {
                self.bind_mut().on_note_event(false, key.into());
            }

            _ => {}
        }

        true
    }
}

impl MidiReceiver {
    pub fn on_note_event(&mut self, on: bool, note: u8) {
        let signal = if on { "note_on" } else { "note_off" };
        self.base_mut().call_deferred("emit_signal", &[
            signal.to_variant(),
            note.to_variant(),
        ]);
    }
}

#[godot_api]
impl MidiReceiver {
    #[signal]
    fn note_on(note: u8);

    #[signal]
    fn note_off(note: u8);
}

#[godot_api]
impl INode for MidiReceiver {}

pub struct BattleTrack {
    pub sheet: Sheet,
    pub ticker: Ticker,
    pub receiver: GdW<MidiReceiver>,
}

impl BattleTrack {
    /// Just pass in the name of the track. No file extension.
    ///
    /// # Memory Leak
    /// See [`BattleTrack::from_godot_path`] docs for more information.
    pub fn new_from_name(track_name: &str) -> BattleTrack {
        let path = format!("res://assets/music/battle/{track_name}.mid");
        let Smf { header, tracks } = Self::from_godot_path(&path);

        let midly::Timing::Metrical(ticks) = header.timing else {
            panic!("P/E/T/S only supports metrical timing MIDI files.");
        };

        let sheet = Sheet::parallel(&tracks);
        let ticker = Ticker::new(ticks.into());
        // let ticker = Ticker::try_from(header.timing).unwrap();
        let receiver = GdW(MidiReceiver::new_alloc());

        BattleTrack {
            sheet,
            ticker,
            receiver,
        }
    }

    /// Parse a MIDI file from a Godot path
    ///
    /// # Memory Leak
    /// Leaks the data buffer from the file.
    /// Don't run this a gazillion times. It's not expected
    /// to have more than like 200 battle themes in the entire game.
    fn from_godot_path<'a>(path: &str) -> Smf<'a> {
        godot_print!("Reading MIDI file: {:?}", path);
        let file = GFile::open(path, ModeFlags::READ);
        let mut file = unwrap_fmt!(file, "Failed to open MIDI file: {}", path);

        let mut data = vec![];
        unwrap_fmt!(
            file.read_to_end(&mut data),
            "Failed to read MIDI file: {}",
            path
        );

        let smf = Smf::parse(data.leak()).expect("Failed to parse MIDI file");
        godot_print!("Successfully read the MIDI file!");

        smf
    }
}
