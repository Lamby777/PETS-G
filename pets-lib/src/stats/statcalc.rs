/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use super::IntegralStat;
use std::{collections::HashMap, rc::Rc};

// function that returns the same type it takes in
pub type StatCalcFn<T> = fn(T) -> T;

// no refcell necessary because they're just functions
pub type CharStatCalcs = HashMap<String, Rc<StatCalcList>>;

/// A list of stat calculation functions for ONE CHARACTER
#[derive(Debug, Clone)]
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

impl Default for StatCalcList {
    fn default() -> Self {
        todo!()
    }
}
