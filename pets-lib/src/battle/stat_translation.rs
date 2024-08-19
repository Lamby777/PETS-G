//!
//! This module is for translating stats from the
//! actual stat numbers into the numbers relevant
//! to the battle engine.
//!

use crate::common::*;
const SPEED_STAT_MULTIPLIER: FloatStat = 800.0;

pub fn speed(speed: IntegralStat) -> FloatStat {
    ((10 + speed) as FloatStat).log2() * SPEED_STAT_MULTIPLIER
}
