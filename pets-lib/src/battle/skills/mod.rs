use crate::consts::battle::*;
use crate::prelude::*;

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
pub(crate) use recovery::RecoverySkill;
pub(crate) use shields::ShieldSkill;

#[typetag::serde(tag = "type")]
pub trait SkillFamily {
    fn name(&self) -> String;
    fn base_cost(&self) -> u32;
    fn description(&self) -> String;

    fn cast(
        &self,
        _caster: Rc<RefCell<dyn Battler>>,
        _targets: Vec<Rc<RefCell<dyn Battler>>>,
        _allies: Vec<Rc<RefCell<dyn Battler>>>,
        _enemies: Vec<Rc<RefCell<dyn Battler>>>,
    );
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
        Element::iter()
            .filter(|v| v.is_magical() && !v.is_unique())
            .collect()
    }

    pub fn list_magical_and_unique() -> Vec<Element> {
        Element::iter().filter(Self::is_magical).collect()
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
    pub fn adjective(&self) -> GString {
        tr(&format!("ELEMENT_ADJ_{:?}", self))
    }
}
