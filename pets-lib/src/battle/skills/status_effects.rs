use super::*;

/// status condition from a skill, and its chances
pub struct EffectPair {
    pub condition: StatusCondition,
    pub chance: ConditionChance,
}

impl EffectPair {
    pub fn describe(&self) -> String {
        let chance = self.chance.describe();
        format!("{} {}", chance, self.condition)
    }
}

pub enum ConditionChance {
    Guaranteed,
    Common,
    Rare,
}

impl ConditionChance {
    /// User-facing string for the chance of a status condition
    /// To be used in skill descriptions
    pub fn describe(&self) -> &str {
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
