use super::skills::Element;
use crate::common::*;

/// The "level" of a type affinity
///
/// Does not have a "normal" 1.0 multiplier, but feel free to add it later
/// There's just no need for it now, but it's not something I'm totally against
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AffinityPower {
    /// The receiver is very weak to this type of damage
    WeakV,

    /// The receiver is weak to this type of damage
    Weak,

    /// The receiver is strong against this type of damage
    Strong,

    /// The receiver is very strong against this type of damage
    StrongV,

    /// The receiver nullifies this type of damage
    Nullify,

    /// The receiver converts this damage into extra health
    Heal,

    /// The receiver reflects this type of damage back to the attacker
    Reflect,
}

impl AffinityPower {
    /// Damage multiplier for enemies receiving damage from this type.
    pub fn _to_damage_multiplier(&self) -> f64 {
        match self {
            Self::WeakV => 1.5,
            Self::Weak => 1.25,
            Self::Strong => 0.5,
            Self::StrongV => 0.25,
            Self::Nullify => 0.0,
            Self::Heal => -0.5,
            Self::Reflect => {
                panic!("attempt to get multiplier of `AffinityPower::Reflect`")
            }
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Affinities(HashMap<Element, AffinityPower>);

impl Affinities {
    pub fn _new(affinities: HashMap<Element, AffinityPower>) -> Self {
        Self(affinities)
    }

    /// Check an affinity. Returns [None] if the affinity is neutral.
    pub fn _get(&self, element: Element) -> Option<AffinityPower> {
        self.0.get(&element).copied()
    }

    /// Basically an equality check for affinities and element lists.
    ///
    /// Only returns `true` if the affinities map contains all the
    /// elements in the list and nothing more.
    pub fn only_has_all_types(&self, types: &[Element]) -> bool {
        let map = &self.0;
        types.len() == map.len() && types.iter().all(|e| map.contains_key(e))
    }
}
