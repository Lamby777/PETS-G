//!
//! this module actually has nothing to do with any
//! type of skill... it's just another place to put
//! status effect data structures for reuse.
//!

use super::*;

/// status effect from a skill, and its chances
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectAndChance {
    pub effect: StatusEffect,
    pub chance: EffectChance,
}

impl EffectAndChance {
    pub fn new(effect: StatusEffect, chance: EffectChance) -> Self {
        Self { effect, chance }
    }
}

impl Describe for EffectAndChance {
    fn describe(&self) -> GString {
        let fx = self.effect.to_string();
        tr_replace! { &self.chance.description_tr_template(); fx }.into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectChance {
    Guaranteed,
    Common,
    Rare,
}

impl EffectChance {
    /// Translation template for the chance of a status
    /// effect to be used in skill descriptions
    pub fn description_tr_template(&self) -> String {
        use EffectChance::*;

        (match self {
            Guaranteed => tr!("SKILL_ATTACK_FX_CHANCE_ALWAYS"),
            Common => tr!("SKILL_ATTACK_FX_CHANCE_HIGH"),
            Rare => tr!("SKILL_ATTACK_FX_CHANCE_LOW"),
        })
        .into()
    }

    pub fn roll(&self) -> bool {
        use EffectChance::*;

        let chance = match self {
            Guaranteed => return true,
            Common => EFFECT_CHANCE_LIKELY,
            Rare => EFFECT_CHANCE_RARE,
        };

        rand::random::<f64>() < chance
    }
}
