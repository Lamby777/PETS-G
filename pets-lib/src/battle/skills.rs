use crate::consts::battle::*;
use crate::prelude::*;

use std::fmt;
use std::time::Duration;

type PowerLevel = u8;

pub enum SkillInfo {
    /// Element-based offensive attack
    /// power: 0 for "status effect only" skills
    Elemental(Element, PowerLevel, Option<EffectPair>),

    /// Heal HP
    Recovery(PowerLevel),

    /// Slow down time
    Flux {
        power: PowerLevel,
        lasts_for: Duration,
    },

    /// Shield
    Shield {
        protects_against: ShieldVariant,
        power: PowerLevel,
        lasts_for: u8,
        partial: bool,
    },
}

pub struct Skill {
    /// Skill info, doesn't matter whether it's attack/heal/support
    pub stats: SkillInfo,

    /// Does this skill affect multiple targets?
    pub plural: bool,

    /// How much (Mana | (B)PP | whatever tf i decide to call it) does it cost?
    pub cost: u32,
}

pub enum ShieldVariant {
    Physical,
    OneElement(Element),
    ManyElements(Vec<Element>),
    AllElements,
}

/// status condition from a skill, and its chances
pub struct EffectPair {
    condition: StatusCondition,
    chance: f32,
}

pub enum ConditionChance {
    Guaranteed,
    Common,
    Rare,
}

impl ConditionChance {
    /// User-facing string for the chance of a status condition
    /// To be used in skill descriptions
    pub fn chance_str(&self) -> &str {
        use ConditionChance::*;

        match self {
            Guaranteed => "Always inflicts",
            Common => "High chance of inflicting",
            Rare => "Low chance of inflicting",
        }
    }
}

impl ConditionChance {
    pub fn roll(&self) -> bool {
        use ConditionChance::*;

        let chance = match self {
            Guaranteed => return true,
            Common => CONDITION_CHANCE_LIKELY,
            Rare => CONDITION_CHANCE_RARE,
        };

        rand::random::<f32>() < chance
    }
}

pub enum Element {
    Fire,
    Ice,
    Electric,
    Wind,
    Earth,
    Psi,

    // Unique
    Fuzz,
    Whip,
}
