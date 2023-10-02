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
    use crate::prelude::IntegralStat;

    macro_rules! identities {
        ($($ty:ty | $name:ident),*) => {
            $(
                pub fn $name(x: IntegralStat) -> $ty {
                    x.into()
                }
            )*
        };
    }

    identities! {
        IntegralStat | max_hp,
        IntegralStat | max_energy,
        IntegralStat | attack,
        IntegralStat | defense,
        IntegralStat | speed,
        IntegralStat | stability,
        IntegralStat | delta,
        IntegralStat | epsilon,
        Option<IntegralStat> | lambda,
        Option<IntegralStat> | max_mana
    }
}
