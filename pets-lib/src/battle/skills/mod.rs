use crate::consts::battle::*;
use crate::prelude::*;

use std::fmt;
use std::time::Duration;

mod status_effects;
use status_effects::*;

mod attack;

#[typetag::serde(tag = "type")]
pub trait SkillFamily {
    fn name(&self) -> &str {
        /// TODO this is only to shut up errors for now
        unimplemented!()
    }

    fn base_cost(&self) -> u32 {
        /// TODO this is only to shut up errors for now
        unimplemented!()
    }

    fn description(&self) -> String;
}

pub enum SkillInfo {
    /// Element-based offensive attack
    /// power: 0 for "status effect only" skills
    Elemental(Element, u8, Option<ChanceOfEffect>),

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

#[derive(Serialize, Deserialize)]
pub struct SkillConcrete {
    /// Skill info, doesn't matter whether it's attack/heal/support
    pub stats: Box<dyn SkillFamily>,

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

#[derive(Debug, Serialize, Deserialize)]
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

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Element {
    /// User-facing string for formatting the element of a skill
    /// Handles the "edge cases" of grammar like "Fuzz" => "Fuzzy"
    pub fn describe(&self) -> String {
        use Element::*;

        match self {
            Electric => "Electric",
            Psi => "Psychic",
            Fuzz => "Fuzzy",
            Whip => "Whip",

            _ => return format!("{}-based", self),
        }
        .to_owned()
    }
}
