use crate::prelude::*;
const SPEED_STAT_MULTIPLIER: FloatStat = 20.0;
const SPEED_STAT_Y_INT: FloatStat = 200.0;

/// player stats -> battle icon stats
pub mod to_battle {
    use super::{SPEED_STAT_MULTIPLIER, SPEED_STAT_Y_INT};
    use crate::prelude::*;

    pub fn speed(speed: IntegralStat) -> FloatStat {
        ((speed as FloatStat) * SPEED_STAT_MULTIPLIER) + SPEED_STAT_Y_INT
    }
}
