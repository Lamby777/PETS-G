/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use super::IntegralStat;
use std::{collections::HashMap, rc::Rc};

// function that returns the same type it takes in
pub type StatCalcFn<T> = Option<fn(T) -> T>;

// no refcell necessary because they're just functions
pub type CharStatCalcs = HashMap<String, Rc<StatCalcList>>;

/// A list of stat calculation functions for ONE CHARACTER
#[derive(Default, Debug, Clone)]
pub struct StatCalcList {
    pub max_hp: StatCalcFn<IntegralStat>,
    pub max_energy: StatCalcFn<IntegralStat>,

    pub attack: StatCalcFn<IntegralStat>,
    pub defense: StatCalcFn<IntegralStat>,
    pub speed: StatCalcFn<IntegralStat>,
    pub stability: StatCalcFn<IntegralStat>, // PK defense

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    pub delta: StatCalcFn<IntegralStat>,   // "the crit one"
    pub epsilon: StatCalcFn<IntegralStat>, // "the combo one"

    // Exclusive to certain characters
    // NOTE maybe use traits for this?
    // idk the overhead of dynamic dispatch might not be worth it
    pub lambda: StatCalcFn<Option<IntegralStat>>,
    pub max_mana: StatCalcFn<Option<IntegralStat>>,
}

macro_rules! use_standard {
    ($($stat:ident),*) => {
        Self {
            $($stat: Some(standard_calcs::$stat)),*
        }
    };
}

impl StatCalcList {
    /// Not to be confused with `default()` from the
    /// `Default` trait, this function returns a calc list
    /// that should be referenced whenever a character's
    /// calc list shows "None"
    pub fn standard() -> Self {
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
                pub fn $name(x: $ty) -> $ty {
                    x
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
