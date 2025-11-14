use crate::common::*;
use crate::consts::battle::*;

use std::fmt;

use godot::prelude::*;
use godot::tools::tr;
use strum::{EnumIter, IntoEnumIterator as _};

mod status_effects;
use status_effects::*;

mod attack;
mod buffs;
mod other;
mod recovery;
mod shields;

pub(crate) use attack::AttackSkill;
pub(crate) use buffs::BuffSkill;
pub(crate) use other::PSIFluxSkill;
pub(crate) use recovery::{RecoverySkill, RecoveryType};
pub(crate) use shields::ShieldSkill;

fn power_to_letter(power: u8) -> GString {
    tr(&format!("SKILL_TIER_{power}"))
}

fn power_to_letter_pl(power: u8, plural: bool) -> GString {
    let power = power_to_letter(power);

    match plural {
        true => tr_replace!("SKILL_POWER_PLURAL"; power).into(),
        false => power,
    }
}

#[typetag::serde(tag = "type")]
pub trait Skill: Debug + Sync + Send {
    fn name(&self) -> String;
    fn base_cost(&self) -> IntegralStat;
    fn description(&self) -> String;

    /// Code that runs when the skill is cast
    fn cast(&self, engine: &mut BattleEngine);
}

// not this "derive everything" crap again... ughhhhhh
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    EnumIter,
    Serialize,
    Deserialize
)]
pub enum Element {
    // Physical
    Blade,   // swords, claws, etc.
    Kinetic, // punches, kicks, etc.
    Pierce,  // bows, guns, etc.

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
        Element::iter()
            .filter(|v| v.is_magical() && !v.is_unique())
            .collect()
    }

    pub fn list_magical_and_unique() -> Vec<Element> {
        Element::iter().filter(Self::is_magical).collect()
    }

    pub fn is_physical(&self) -> bool {
        use Element::*;
        matches!(self, Blade | Kinetic | Pierce)
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
    pub fn adjective(&self) -> GString {
        tr(&format!("ELEMENT_ADJ_{self:?}"))
    }
}
