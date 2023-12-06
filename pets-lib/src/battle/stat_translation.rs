//!
//! This module is for translating stats from the
//! actual stat numbers into the numbers relevant
//! to the battle engine.
//!

use crate::prelude::*;
const SPEED_STAT_MULTIPLIER: FloatStat = 200.0;

pub fn speed(speed: IntegralStat) -> FloatStat {
    // oh boy, linear algebra! maybe school DID actually teach me something
    // TODO maybe use logarithms, refer to that one graph i made a while ago
    // ofc,, past me didn't know about the "units" in godot, so values might
    // need to be adjusted for speed and similar implementation-heavy stats
    (speed as FloatStat) * SPEED_STAT_MULTIPLIER
}
