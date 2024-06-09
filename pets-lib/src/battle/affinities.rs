use godot::prelude::*;

use super::skills::Element;
use crate::prelude::*;

/// What kind of damage does the shield block?
#[derive(Serialize, Deserialize)]
pub enum Affinities {
    Physical,
    Magical,
    AllElements,

    Specific(Vec<Element>),
}

impl Affinities {
    /// Attempts to convert an explicit list of elements like
    /// [Blade, Kinetic] into a more general variant like Physical
    pub fn coerce_specific(&self) -> Option<Self> {
        use Affinities::*;

        let Specific(elements) = self else {
            panic!("attempt to `coerce_specific` a non-specific shield");
        };

        // we don't need to sort the other vectors we're comparing to,
        // because sorting this one will sort based on enum variant order
        // and the enum iterators happen to also iterate in order...
        // (at least i sure hope they do)
        let mut sorted = elements.clone();
        sorted.sort();

        Some(if sorted == Element::list_all() {
            AllElements
        } else if sorted == Element::list_physical() {
            Physical
        } else if sorted == Element::list_magical() {
            Magical
        } else {
            return None;
        })
    }
}

impl Describe for Affinities {
    fn describe(&self) -> GString {
        use Affinities::*;

        // TODO use `tr`
        match self {
            Specific(_) => "Specialized",
            AllElements => "Almighty",
            Magical => "Magical",
            Physical => "Physical",
        }
        .into()
    }
}
