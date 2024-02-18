use super::*;
use std::time::Duration;

/// What kind of damage does the shield block?
#[derive(Serialize, Deserialize)]
pub enum ShieldAffinity {
    Elements { elements: Vec<Element> },
    AllElements,
}

impl ShieldAffinity {
    fn describe_affinity(&self) -> String {
        use ShieldAffinity::*;

        match self {
            AllElements => "all kinds of",

            Elements { elements } => {
                // if only blade and kinetic, call it physical
                if elements.len() == 2
                    && elements.contains(&Element::Blade)
                    && elements.contains(&Element::Kinetic)
                {
                    return "physical".to_owned();
                }

                let iter = elements.iter().map(|x| x.describe());
                return join_words(iter, "and")
                    .expect("shield of many elements has empty block list");
            }
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    pub protects_against: ShieldAffinity,

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
        let affinity = self.protects_against.describe_affinity();

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

    #[test]
    fn test_describe_impenetrable_flawless() {
        let skill = ShieldSkill {
            protects_against: ShieldAffinity::AllElements,
            hits: 1,
            multiplier: 0.5,
            reflect: false,
        };

        assert_eq!(
            skill.description(),
            "Casts a sturdy shield that blocks all kinds of damage once."
        );
    }
}
