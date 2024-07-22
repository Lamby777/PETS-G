// (warning: link may be outdated)
// <https://www.desmos.com/calculator/2lpxhqyj7l>
//
// For all these functions, the doc comment is the
// mathematical function. The variable `x` represents
// the variable `lvl` in our code.
use crate::prelude::*;

pub fn level_to_stats(lvl: IntegralStat) -> InherentStats {
    InherentStats {
        max_hp: max_hp(lvl),
        max_energy: max_energy(lvl),
        attack: attack(lvl),
        defense: defense(lvl),
        speed: speed(lvl),
        stability: stability(lvl),
        delta: delta(lvl),
        epsilon: epsilon(lvl),
        lambda: lambda(lvl),
        max_mana: max_mana(lvl),
    }
}

/// floor(    ( (log(x) + x^3) / 16 )   + x )
pub fn _xp_requirement(lvl: IntegralStat) -> IntegralStat {
    // NOTE x is the level which reaching this amount of xp
    // will get you to, and NOT the level you're currently at
    let p1 = (lvl as FloatStat).log10() + (lvl.pow(3) as FloatStat);

    // division is done after floor, because it
    // would have been floored anyway (i think)
    let quotient = FloatStat::floor(p1) as IntegralStat / 16;

    quotient + lvl
}

/// floor(5 * log_1.4_(x+1)) + 0.5x + 20
pub fn max_hp(lvl: IntegralStat) -> IntegralStat {
    let p1 = (5.0 * ((lvl + 1) as FloatStat).log(1.4)) as IntegralStat;
    let p2 = lvl / 2;

    p1 + p2 + 20
}

/// 10 + (floor(x/10) * 10)
pub fn max_energy(lvl: IntegralStat) -> IntegralStat {
    // for this, we don't need floor in practice because
    // rust's integer division does that for us
    10 + ((lvl / 10) * 10)
}

pub fn attack(lvl: IntegralStat) -> IntegralStat {
    // (not listed in desmos link)
    lvl
}

pub fn defense(lvl: IntegralStat) -> IntegralStat {
    // (not listed in desmos link)
    lvl
}

/// floor(25 * ln(x))
pub fn speed(lvl: IntegralStat) -> IntegralStat {
    (25.0 * (lvl as FloatStat).ln()) as IntegralStat
}

/// ceil(  -((x-60)/14)^2  ) + 110
pub fn stability(lvl: IntegralStat) -> IntegralStat {
    let p1 = lvl as FloatStat - 60.0 / 14.0;
    let p1 = -p1.powi(2);

    110 + p1 as IntegralStat
}

/// floor(10 * sqrt(x))
pub fn delta(lvl: IntegralStat) -> IntegralStat {
    (10.0 * (lvl as FloatStat).sqrt()) as IntegralStat
}

pub fn epsilon(_lvl: IntegralStat) -> IntegralStat {
    // NOTE this one is mostly upgraded manually,
    // but should still improve somewhat with levels...
    1
}

// -----------------------------------------------------
// Characters don't have the following stats by default,
// -----------------------------------------------------

pub fn lambda(_lvl: IntegralStat) -> Option<IntegralStat> {
    None
}

pub fn max_mana(_lvl: IntegralStat) -> Option<IntegralStat> {
    // NOTE use floor( (z/12)^2.5 ) to calculate for characters
    // that DO have PK abilities... but this is the default so nah
    None
}
