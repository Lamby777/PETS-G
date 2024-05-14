//!
//! Rhythms for the battle system...
//!
//! EDIT: We're using MIDI now.
//! No more reinventing the wheel.
//! No fun allowed.
//!

use derived_deref::*;
use midly::Smf;

#[derive(Deref, DerefMut)]
pub struct PetsRhythm<'a>(Smf<'a>);

impl PetsRhythm<'_> {
    pub fn new(smf: Smf) -> Self {
        Self(smf)
    }

    /// Give a time and this function will find you the first
    /// note before that position in the track.
    pub fn last_played_note(smf: &Smf, time: u32) -> ! {
        let beat = beat_track(smf);

        todo!()
    }

    // getter for finding the beat track out of all the tracks
    fn beat_track(smf: &Smf) -> ! {
        todo!()
    }
}
