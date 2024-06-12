use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub requirements: Vec<Box<dyn Requirement>>,
}

#[typetag::serde(tag = "type")]
pub trait Requirement {
    fn fulfilled(&self) -> bool;
}
