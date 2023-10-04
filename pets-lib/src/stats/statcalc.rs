/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use crate::prelude::*;

pub type CharStatCalcs = HashMap<String, Rc<StatCalcList>>;

// function that returns the same type it takes in
#[derive(Debug, Clone)]
pub struct StatCalcFn<T> {
    pub calc: fn(IntegralStat) -> T,
}

impl<T> Deref for StatCalcFn<T> {
    type Target = fn(IntegralStat) -> T;

    fn deref(&self) -> &Self::Target {
        &self.calc
    }
}

impl<T> From<fn(IntegralStat) -> T> for StatCalcFn<T> {
    fn from(calc: fn(IntegralStat) -> T) -> Self {
        Self { calc }
    }
}

/// A list of stat calculation functions for ONE CHARACTER
#[derive(Debug, Clone)]
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
    ($($stat:ident),*) => {
        Self {
            $($stat: StatCalcFn::from(standard_calcs::$stat as fn(_) -> _)),*
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
            max_mana
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
    pub fn max_hp(lvl: IntegralStat) -> IntegralStat {
        let p1 = FloatStat::floor(5.0 * (lvl as FloatStat).log(1.4));
        let p2 = (0.5 * lvl as FloatStat) + 40.0;

        (p1 + p2) as IntegralStat
    }

    /// 10 + (floor(x/10) * 10)
    pub fn max_energy(lvl: IntegralStat) -> IntegralStat {
        // for this, we don't need floor in practice because
        // rust's integer division does that for us
        10 + ((lvl / 10) * 10)
    }

    pub fn attack(_lvl: IntegralStat) -> IntegralStat {
        todo!()
    }

    pub fn defense(_lvl: IntegralStat) -> IntegralStat {
        todo!()
    }

    /// floor(25 * ln(x))
    pub fn speed(lvl: IntegralStat) -> IntegralStat {
        FloatStat::floor(25.0 * (lvl as FloatStat).ln()) as IntegralStat
    }

    pub fn stability(_lvl: IntegralStat) -> IntegralStat {
        todo!()
    }

    pub fn delta(_lvl: IntegralStat) -> IntegralStat {
        todo!()
    }

    pub fn epsilon(_lvl: IntegralStat) -> IntegralStat {
        todo!()
    }

    pub fn lambda(_lvl: IntegralStat) -> Option<IntegralStat> {
        todo!()
    }

    pub fn max_mana(_lvl: IntegralStat) -> Option<IntegralStat> {
        todo!()
    }
}
