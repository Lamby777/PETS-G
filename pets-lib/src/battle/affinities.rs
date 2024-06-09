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

    pub fn describe_shield(&self) -> GString {
        "Specialized".into()
    }
}

impl Describe for Affinities {
    fn describe(&self) -> GString {
        todo!()
    }
}
