/// Function that calculates a stat for a character
/// Basically just "base stats" as a function
use super::IntegralStat;

pub type StatCalcFn<T> = fn(T) -> T;

/// A list of stat calculation functions for ONE CHARACTER
/// I've probably made this same mistake of thinking this is
/// for all characters MANY times now. It is only for one.
#[derive(Debug, Clone)]
pub struct CharStatCalcs {
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
