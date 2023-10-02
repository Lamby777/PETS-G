use crate::prelude::*;
const SPEED_STAT_MULTIPLIER: FloatStat = 200.0;

/// player stats -> battle icon stats
pub mod to_battle {
    use super::SPEED_STAT_MULTIPLIER;
    use crate::prelude::*;

    pub fn speed(speed: IntegralStat) -> FloatStat {
        // oh boy, linear algebra! maybe school DID actually teach me something
        // TODO maybe use logarithms, refer to that one graph i made a while ago
        // ofc,, past me didn't know about the "units" in godot, so values might
        // need to be adjusted for speed and similar implementation-heavy stats
        (speed as FloatStat) * SPEED_STAT_MULTIPLIER
    }
}
