//!
//! Rhythms for the battle system...
//!
//! These type defs will probably also be used for
//! developing an open-source P/E/T/S rhythm editor.
//!

// TODO add a way to deserialize from file, to allow for
// custom rhythms and modding.

/// Things that can happen within a track section...
/// Right now, it's just a note, but maybe...
enum RhythmEvent {
    /// A note that the player must hit
    Note {
        /// Delay after this note, before going on to the
        /// next one (delay is in notes, not seconds)
        length: u8,
    },
}

/// "Parts" of a track. Tracks can switch tempo
struct RhythmSection {
    start_time: f64,          // Start time of the section in seconds
    end_time: f64,            // End time of the section in seconds
    tempo: f64,               // Tempo in beats per minute
    events: Vec<RhythmEvent>, // List of events within this section/
}

/// One track's information, for rhythm purposes
///
/// Ideally, we wouldn't have to do it, but maybe
/// it would be easier to just Box::leak() this if
/// it gets too complicated to worry about lifetimes.
///
/// The only reason this is a runtime thing is to
/// make things easier for modders. Making rhythms
/// hard-coded in the compiled binary would kinda
/// suck for them. It might even be better performance-wise
/// to just leak all the tracks at load-time instead of
/// having them lazily loaded, but we'll see.
struct TrackRhythm<'a> {
    name: String,
    creator: String,

    /// Split sections of a track, for tempo shifts and
    /// potentially other stuff in the future.
    // Since sections can repeat, we don't want to .clone()
    // all over the place for repetitive stuff.
    section: Vec<&'a RhythmSection>,
}
