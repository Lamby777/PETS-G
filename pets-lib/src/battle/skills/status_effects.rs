//!
//! this module actually has nothing to do with any
//! type of skill... it's just another place to put
//! status effect data structures for reuse.
//!

use super::*;

/// status effect from a skill, and its chances
#[derive(Serialize, Deserialize)]
pub struct ChanceOfEffect {
    pub effect: StatusEffect,
    pub chance: EffectChance,
}

impl ChanceOfEffect {
    pub fn new(effect: StatusEffect, chance: EffectChance) -> Self {
        Self { effect, chance }
    }

    pub fn describe(&self) -> String {
        let template = self.chance.description_tr_template();
        template.replace("{fx}", &self.effect.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub enum EffectChance {
    Guaranteed,
    Common,
    Rare,
}

impl EffectChance {
    /// Translation template for the chance of a status
    /// effect To be used in skill descriptions
    pub fn description_tr_template(&self) -> String {
        use EffectChance::*;

        (match self {
            Guaranteed => tr!("SKILL_ATTACK_FX_CHANCE_ALWAYS"),
            Common => tr!("SKILL_ATTACK_FX_CHANCE_HIGH"),
            Rare => tr!("SKILL_ATTACK_FX_CHANCE_LOW"),
        })
        .into()
    }
}

impl EffectChance {
    pub fn _roll(&self) -> bool {
        use EffectChance::*;

        let chance = match self {
            Guaranteed => return true,
            Common => EFFECT_CHANCE_LIKELY,
            Rare => EFFECT_CHANCE_RARE,
        };

        rand::random::<f64>() < chance
    }
}
