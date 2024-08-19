use super::skills::Element;
use crate::common::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AffinityPower {
    /// The receiver is weak to this type of damage
    Weak,

    /// The receiver is strong against this type of damage
    Strong,

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
            Self::Weak => 2.0,
            Self::Strong => 0.5,
            Self::Nullify => 0.0,
            Self::Heal => -0.5,
            Self::Reflect => {
                panic!("attempt to get multiplier of `AffinityPower::Reflect`")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Affinities(HashMap<Element, AffinityPower>);

impl Default for Affinities {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Affinities {
    pub fn _new(affinities: HashMap<Element, AffinityPower>) -> Self {
        Self(affinities)
    }

    /// Check an affinity. Returns `None` if the affinity is neutral.
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
