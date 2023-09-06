//!
//! Rhythms for the battle system...
//!
//! These type defs will probably also be used for
//! developing an open-source P/E/T/S rhythm editor.
//!

// TODO add a way to deserialize from file, to allow for
// custom rhythms and modding.

//! Rhythms are made up of notes and tempo changes.
enum RhythmComponent {
    Note { length: u8 },
    TempoChange { length: u8, bpm: f64 },
}

// struct TrackRhythm {
//     components: Vec<RhythmComponent>,
// }

const SIMPLE_BEAT: &[RhythmComponent] = &[
    RhythmComponent::Note { length: 8 },
    RhythmComponent::Note { length: 8 },
    RhythmComponent::Note { length: 8 },
    RhythmComponent::Note { length: 8 },
];
