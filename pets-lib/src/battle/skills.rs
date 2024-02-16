use crate::consts::battle::*;
use crate::prelude::*;

pub struct Skill {
    pub family: SkillFamily,
    pub to_all: bool,
    pub power: u8,

    pub cost: u32,
    pub effect: Option<SkillEffect>,
}

/// status condition chances
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

pub enum SkillFamily {
    // Elemental
    Fire,
    Freeze,
    Thunder,
    Wind,

    // Unique
    Fuzz,
    Whip,
}
