use super::*;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum ShieldVariant {
    Physical,
    OneElement { element: Element },
    ManyElements { elements: Vec<Element> },
    AllElements,
}

impl ShieldVariant {
    fn describe_affinity(&self) -> String {
        use ShieldVariant::*;

        match self {
            Physical => "physical",
            OneElement { element } => return element.describe(),

            ManyElements { elements } => {
                let iter = elements.iter().map(|x| x.describe());
                return join_words(iter, "and")
                    .expect("shield of many elements with empty block list");
            }

            AllElements => "all kinds of",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    pub protects_against: ShieldVariant,

    /// How many hits the shield can take
    pub hits: u8,

    /// Percent of damage that gets through
    /// If zero, it blocks all damage
    pub multiplier: f32,

    /// Whether the shield reflects damage
    pub reflect: bool,
}

impl ShieldSkill {
    // it's not gonna become a hard error, stfu clippy
    // <https://github.com/rust-lang/rust/issues/41620#issuecomment-1722194944>
    #[allow(illegal_floating_point_literal_pattern)]
    fn multi_to_str(multi: f32) -> &'static str {
        match multi {
            // ranges because multiple shields combine their power
            0.0 => "impenetrable ",
            0.0..=0.2 => "formidable ",
            0.4..=0.6 => "sturdy ",
            0.6..=0.8 => "",
            0.8..=1.0 => "weak ",

            // if your shield powers have been weakened...
            1.0.. => "nullified ",
            _ => unreachable!("shield has negative multiplier"),
        }
    }

    fn hits_to_str(hits: u8) -> &'static str {
        match hits {
            1 => "single",
            2 => "double",
            3 => "triple",
            4 => "quadruple",
            5 => "quintuple",
            _ => "multiple",
        }
    }
}

#[typetag::serde]
impl SkillFamily for ShieldSkill {
    fn description(&self) -> String {
        let potency = ShieldSkill::multi_to_str(self.multiplier);
        let reflectivity = if self.reflect { "reflects" } else { "blocks" };
        // let affinity = self.

        format!(
            // "Casts a {} shield that {} {} damage {}.",
            "Casts a {} shield that {}.",
            potency, reflectivity,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_describe_impenetrable_flawless() {
        let skill = ShieldSkill {
            protects_against: ShieldVariant::AllElements,
            hits: 1,
            multiplier: 0.5,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            "Casts a sturdy shield that blocks all damage once."
        );
    }
}
