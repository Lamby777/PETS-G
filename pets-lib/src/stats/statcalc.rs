/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use crate::prelude::*;

pub type CharStatCalcs = HashMap<String, Rc<StatCalcList>>;

/// Function that computes a stat of type T, given a level
pub type StatCalcFn<T> = fn(IntegralStat) -> T;

/// A list of stat calculation functions for ONE CHARACTER
#[allow(unused)]
pub struct StatCalcList {
    pub xp_requirement: StatCalcFn<IntegralStat>,

    pub max_hp: StatCalcFn<IntegralStat>,
    pub max_energy: StatCalcFn<IntegralStat>,

    pub attack: StatCalcFn<IntegralStat>,
    pub defense: StatCalcFn<IntegralStat>,
    pub speed: StatCalcFn<IntegralStat>,
    pub stability: StatCalcFn<IntegralStat>,

    pub delta: StatCalcFn<IntegralStat>,
    pub epsilon: StatCalcFn<IntegralStat>,

    pub lambda: StatCalcFn<Option<IntegralStat>>,
    pub max_mana: StatCalcFn<Option<IntegralStat>>,
}

macro_rules! use_standard {
    ($($stat:ident),* $(,)?) => {
        Self {
            $($stat: standard_calcs::$stat as fn(_) -> _),*
        }
    };
}

impl Default for StatCalcList {
    fn default() -> Self {
        use_standard! {
            xp_requirement,
            max_hp,
            max_energy,
            attack,
            defense,
            speed,
            stability,
            delta,
            epsilon,
            lambda,
            max_mana,
        }
    }
}

mod standard_calcs {
    //! (warning: link may be outdated)
    //! <https://www.desmos.com/calculator/2lpxhqyj7l>
    //!
    //! For all these functions, the doc comment is the
    //! mathematical function. The variable `x` represents
    //! the variable `lvl` in our code.
    // TODO stop using `as` to cast... somewhat unsafe

    use crate::prelude::{FloatStat, IntegralStat};

    /// floor(    ( (log(x) + x^3) / 16 )   + x )
    pub fn xp_requirement(lvl: IntegralStat) -> IntegralStat {
        // NOTE x is the level which reaching this amount of xp
        // will get you to, and NOT the level you're currently at
        let p1 = (lvl as FloatStat).log10() + (lvl.pow(3) as FloatStat);

        // division is done after floor, because it
        // would have been floored anyway (i think)
        let quotient = FloatStat::floor(p1) as IntegralStat / 16;

        quotient + lvl
    }

    /// floor(5 * log_1.4_(x)) + 0.5x + 40
    // TODO idk what i was thinking... maybe use sqrt growth instead of log
    pub fn max_hp(lvl: IntegralStat) -> IntegralStat {
        let p1 = (5.0 * (lvl as FloatStat).log(1.4)) as IntegralStat;
        let p2 = (lvl / 2) + 40;

        p1 + p2
    }

    /// 10 + (floor(x/10) * 10)
    pub fn max_energy(lvl: IntegralStat) -> IntegralStat {
        // for this, we don't need floor in practice because
        // rust's integer division does that for us
        10 + ((lvl / 10) * 10)
    }

    pub fn attack(lvl: IntegralStat) -> IntegralStat {
        // TODO think of a better formula
        // (not listed in desmos link)
        lvl
    }

    pub fn defense(lvl: IntegralStat) -> IntegralStat {
        // TODO think of a better formula
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
        //
        // TODO prob needs its own dedicated calculation function
        // in a separate file... but for now let's just leave it
        // as always returning 1
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
}
