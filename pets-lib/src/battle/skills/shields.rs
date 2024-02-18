use super::*;
use std::time::Duration;

/// What kind of damage does the shield block?
#[derive(Serialize, Deserialize)]
pub enum ShieldAffinity {
    Physical,
    Magical,
    AllElements,

    Specific(Vec<Element>),
}

impl ShieldAffinity {
    pub fn from_vec(elements: Vec<Element>) -> ShieldAffinity {
        use ShieldAffinity::*;

        fn matches(slice: &[Element], pred: impl Fn(&Element) -> bool) -> bool {
            slice.iter().any(pred)
        }

        if matches(&elements, Element::is_physical) {
            Physical
        } else if matches(&elements, Element::is_magical) {
            Magical
        } else if elements.len() == std::mem::variant_count::<Element>() {
            AllElements
        } else {
            Specific(elements)
        }
    }

    /// convert to list of elements, if it isn't already
    pub fn to_vec(elements: ShieldAffinity) -> Vec<Element> {
        use ShieldAffinity::*;

        match elements {
            AllElements => Element::list_all(),
            Magical => Element::list_magical(),
            Physical => Element::list_physical(),
            Specific(inner) => inner,
        }
    }

    fn describe_affinity(&self) -> String {
        use ShieldAffinity::*;

        match self {
            AllElements => "all kinds of",
            Magical => "magical",
            Physical => "physical",

            Specific(elements) => {
                let iter = elements.iter().map(|x| x.describe());
                return join_words(iter, "and", Some("only"))
                    .expect("shield of many elements has empty block list");
            }
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    pub affinity: ShieldAffinity,

    /// How many hits the shield can take
    pub hits: u8,

    /// Percent of damage that gets through
    /// If zero, it blocks all damage
    pub multiplier: f64,

    /// Whether the shield reflects damage
    pub reflect: bool,
}

impl ShieldSkill {
    // it's not gonna become a hard error, stfu clippy
    // <https://github.com/rust-lang/rust/issues/41620#issuecomment-1722194944>
    #[allow(illegal_floating_point_literal_pattern)]
    fn multi_to_str(multi: f64) -> &'static str {
        match multi {
            // ranges because multiple shields combine their power
            0.0 => "impenetrable ",
            0.0..=0.3 => "sturdy ",
            0.3..=0.7 => "",
            0.7..=1.0 => "weak ",

            // if your shield powers have been weakened...
            1.0.. => "nullified ",
            _ => unreachable!("shield has negative multiplier"),
        }
    }

    fn hits_to_str(hits: u8) -> &'static str {
        match hits {
            0 => unreachable!("shield that can't withstand any hits"),
            1 => "once",
            2..=3 => "a couple times",
            4..=6 => "several times",
            7..=10 => "many times",
            11..=15 => "for a while",
            _ => unreachable!("shield that can withstand over 15 hits"),
        }
    }
}

#[typetag::serde]
impl SkillFamily for ShieldSkill {
    fn description(&self) -> String {
        let potency = ShieldSkill::multi_to_str(self.multiplier);
        let reflectivity = if self.reflect { "reflects" } else { "blocks" };
        let affinity = self.affinity.describe_affinity();

        let part1 = format!(
            "Casts a {}shield that {} {} damage",
            potency, reflectivity, affinity
        );

        match self.hits {
            0 => format!("{}. It probably won't last...", potency),
            hits => {
                let hits = ShieldSkill::hits_to_str(hits);
                format!("{} {}.", part1, hits)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Element::*;

    #[test]
    fn test_describe_impenetrable_all_elements() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::AllElements,
            hits: 1,
            multiplier: 0.2,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            "Casts a sturdy shield that blocks all kinds of damage once."
        );
    }

    #[test]
    fn test_describe_7hit_reflective_magical() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::Magical,
            hits: 7,
            multiplier: 0.8,
            reflect: true,
        };

        assert_eq!(
            skill.description(),
            "Casts a weak shield that reflects magical damage many times."
        );
    }

    #[test]
    fn test_describe_two_specific_elements() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::Specific(vec![Psi, Spirit]),
            hits: 3,
            multiplier: 0.5,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            "Casts a shield that blocks Psychic and Supernatural damage a couple times."
        );
    }

    #[test]
    fn test_describe_many_specific_elements() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::Specific(vec![Fire, Psi, Ice, Spirit]),
            hits: 3,
            multiplier: 0.5,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            indoc! {"Casts a shield that blocks Fire-based, Psychic, \
            Ice-based, and Supernatural damage a couple times."}
        );
    }

    #[test]
    fn test_describe_one_specific_element() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::Specific(vec![Fuzz]),
            hits: 3,
            multiplier: 0.5,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            "Casts a shield that blocks only Fuzzy damage a couple times."
        );
    }
}
