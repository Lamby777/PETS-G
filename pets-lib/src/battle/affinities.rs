use godot::prelude::*;

use super::skills::Element;
use crate::prelude::*;

pub enum AffinityPower {
    Reflect,
    Null,
    Strong,
    Weak,
}

#[derive(Serialize, Deserialize)]
pub struct Affinities {
    inner: Vec<Element>,
}

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
