use godot::prelude::*;

use super::skills::Element;
use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AffinityPower {
    Weak,
    Strong,
    Nullify,
    Reflect,
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

    pub fn describe_damage_blocked(&self) -> GString {
        if self.is_physical_shield() {
            return tr("SKILL_SHIELD_PHYSICAL_DESC");
        }

        if self.is_magical_shield() {
            return tr("SKILL_SHIELD_MAGICAL_DESC");
        }

        if self.is_unique_shield() {
            return tr("SKILL_SHIELD_UNIQUE_DESC");
        }

        tr("SKILL_SHIELD_SPECIALIZED_DESC")
    }

    fn has_all_types(&self, types: &[Element]) -> bool {
        let map = &self.0;
        types.len() == map.len() && types.iter().all(|e| map.contains_key(e))
    }

    fn is_physical_shield(&self) -> bool {
        self.has_all_types(&Element::list_physical())
    }

    fn is_magical_shield(&self) -> bool {
        self.has_all_types(&Element::list_magical())
    }

    fn is_unique_shield(&self) -> bool {
        self.has_all_types(&Element::list_magical_and_unique())
    }
}
