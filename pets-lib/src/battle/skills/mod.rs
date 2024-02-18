use crate::consts::battle::*;
use crate::prelude::*;

use std::fmt;
use std::time::Duration;

mod status_effects;
use status_effects::*;

mod attack;

pub trait Skill {
    fn name(&self) -> &str;
    fn description(&self) -> String;
    fn cost(&self) -> u32;
}

pub enum SkillInfo {
    /// Element-based offensive attack
    /// power: 0 for "status effect only" skills
    Elemental(Element, u8, Option<EffectPair>),

    /// Heal HP
    Recovery(u8),

    /// Slow down time
    Flux { power: u8, lasts_for: Duration },

    /// Shield
    Shield {
        protects_against: ShieldVariant,
        power: u8,
        lasts_for: u8,
        partial: bool,
    },
}

pub struct SkillConcrete {
    /// Skill info, doesn't matter whether it's attack/heal/support
    pub stats: &'static dyn Skill,

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
