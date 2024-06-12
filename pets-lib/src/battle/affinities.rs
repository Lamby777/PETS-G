use super::skills::Element;
use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AffinityPower {
    Weak,
    Strong,
    Nullify,
    Heal,
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

#[derive(Serialize, Deserialize)]
pub struct Affinities(HashMap<Element, AffinityPower>);

impl Default for Affinities {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Affinities {
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
