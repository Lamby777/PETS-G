/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use super::IntegralStat;
use std::collections::HashMap;

// function that returns the same type it takes in
pub type StatCalcFn<T> = Option<fn(T) -> T>;

// no refcell necessary because they're just functions
pub type CharStatCalcs = HashMap<String, StatCalcList>;

/// A list of stat calculation functions for ONE CHARACTER
#[derive(Default, Debug, Clone)]
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
