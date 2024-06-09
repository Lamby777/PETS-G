use godot::prelude::*;

use super::skills::Element;
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum AffinityPower {
    Reflect,
    Null,
    Strong,
    Weak,
}

#[derive(Serialize, Deserialize)]
pub struct Affinities(pub HashMap<Element, AffinityPower>);

impl Affinities {
    pub fn describe_shield(&self) -> GString {
        "Specialized".into()
    }
}

impl Describe for Affinities {
    fn describe(&self) -> GString {
        todo!()
    }
}
