use crate::consts::battle::*;
use crate::prelude::*;

use std::fmt;

use strum::{EnumIter, IntoEnumIterator};

mod status_effects;
use status_effects::*;

mod attack;
mod recovery;
mod shields;

#[typetag::serde(tag = "type")]
pub trait SkillFamily {
    fn name(&self) -> &str {
        // TODO this is only to shut up errors for now
        unimplemented!()
    }

    fn base_cost(&self) -> u32 {
        // TODO this is only to shut up errors for now
        unimplemented!()
    }

    fn description(&self) -> String;
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

#[derive(Clone, Debug, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Element {
    // Physical
    Blade,   // swords, claws, etc.
    Kinetic, // punches, guns, etc.

    // Elemental
    Fire,
    Ice,      // or water-related
    Electric, // or magnetic, nuclear, etc.
    Wind,     // or anything gas-based
    Earth,    // or anything rock-based, like sand or metal
    Psi,
    Spirit, // supernatural, ghostly, etc.

    // Unique
    Fuzz, // Ethan's magic, often causes confusion/sleep
    Whip, // Siva's magic, often causes paralysis/bleeding
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Element {
    pub fn list_all() -> Vec<Element> {
        Element::iter().collect()
    }

    pub fn list_physical() -> Vec<Element> {
        Element::iter().filter(Self::is_physical).collect()
    }

    pub fn list_magical() -> Vec<Element> {
        Element::iter().filter(Self::is_magical).collect()
    }

    /// Skips unique elements (Fuzz, Whip, etc.)
    pub fn list_magical_not_unique() -> Vec<Element> {
        Element::iter()
            .filter(Self::is_magical)
            .filter(|v| !v.is_unique())
            .collect()
    }

    pub fn list_unique() -> Vec<Element> {
        Element::iter().filter(Self::is_unique).collect()
    }

    pub fn list_not_unique() -> Vec<Element> {
        Element::iter().filter(|v| !v.is_unique()).collect()
    }

    pub fn is_physical(&self) -> bool {
        use Element::*;
        matches!(self, Blade | Kinetic)
    }

    pub fn is_magical(&self) -> bool {
        !self.is_physical()
    }

    pub fn is_unique(&self) -> bool {
        use Element::*;
        matches!(self, Fuzz | Whip)
    }

    /// User-facing string for formatting the element of a skill
    /// Handles the "edge cases" of grammar like "Fuzz" => "Fuzzy"
    pub fn describe(&self) -> String {
        use Element::*;

        match self {
            Blade => "Slash",
            Kinetic => "Kinetic",

            Electric => "Electric",
            Psi => "Psychic",
            Spirit => "Supernatural",

            Fuzz => "Fuzzy",
            Whip => "Whip",

            _ => return format!("{}-based", self),
        }
        .to_owned()
    }
}
