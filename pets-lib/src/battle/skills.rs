use crate::consts::battle::*;
use crate::prelude::*;

use std::fmt;

type PowerLevel = u8;

pub enum SkillInfo {
    /// Element-based offensive attack
    /// power: 0 for "status effect only" skills
    Elemental(Element, PowerLevel, Option<SkillEffect>),

    /// Heal HP
    Recovery(PowerLevel),
}

pub struct Skill {
    pub stats: SkillInfo,
    pub to_all: bool,
    pub cost: u32,
}

/// status condition from a skill, and its chances
pub struct SkillEffect {
    condition: StatusCondition,
    chance: f32,
}

pub enum ConditionChance {
    Guaranteed,
    Common,
    Rare,
}

impl ConditionChance {
    /// User-facing string for the chance of a status condition to
    /// be used in skill descriptions
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
    Freeze,
    Electric,
    Wind,
    Earth,
    Psi,

    // Unique
    Fuzz,
    Whip,
}
