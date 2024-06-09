use godot::builtin::GString;

use crate::battle::affinities::Affinities;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    ///
    /// Use `set_affinity` to set this, not directly.
    /// The setter will handle converting lists of specific
    /// elements into more general affinities where possible.
    pub affinity: Affinities,

    /// How many hits the shield can take
    pub hits: u8,

    /// Percent of damage that gets through
    /// If zero, it blocks all damage
    pub multiplier: f64,

    /// Whether the shield reflects damage
    pub reflect: bool,

    /// Does the shield cover the whole party?
    pub plural: bool,
}

impl ShieldSkill {
    fn multi_description(multi: f64) -> GString {
        if multi < 0.0 {
            panic!("shield with negative multiplier");
        }

        let key = if multi == 0.0 {
            "SKILL_SHIELD_POTENCY_IMPENETRABLE"
        } else if multi <= 0.3 {
            "SKILL_SHIELD_POTENCY_STURDY"
        } else if multi <= 0.7 {
            "SKILL_SHIELD_POTENCY_FAIR"
        } else if multi <= 1.0 {
            "SKILL_SHIELD_POTENCY_WEAK"
        } else {
            // if your shield powers have been weakened past 1.0...
            "SKILL_SHIELD_POTENCY_NULLIFIED"
        };

        tr(key)
    }

    fn hits_to_str(hits: u8) -> GString {
        tr(match hits {
            0 => "SKILL_SHIELD_HITS_NONE",
            1 => "SKILL_SHIELD_HITS_ONE",
            2..=3 => "SKILL_SHIELD_HITS_COUPLE",
            4..=6 => "SKILL_SHIELD_HITS_SEVERAL",
            7..=10 => "SKILL_SHIELD_HITS_MANY",
            11.. => "SKILL_SHIELD_HITS_WHILE",
            // _ => unreachable!("shield that can withstand over 15 hits"),
        })
    }

    pub fn set_affinity(&mut self, aff: Affinities) {
        self.affinity = aff.coerce_specific().unwrap_or(aff);
    }

    /// "Wide" or "Narrow"
    fn shield_width_str(&self) -> GString {
        tr(match self.plural {
            true => "SKILL_SHIELD_WIDTH_WIDE",
            false => "SKILL_SHIELD_WIDTH_NARROW",
        })
    }

    /// "Shield" or "Mirror"
    fn shield_type_str(&self) -> GString {
        tr(match self.reflect {
            true => "SKILL_SHIELD_NAME_MIRROR",
            false => "SKILL_SHIELD_NAME_SHIELD",
        })
    }
}

#[typetag::serde]
impl SkillFamily for ShieldSkill {
    fn name(&self) -> String {
        let name = self.shield_type_str();
        let width = self.shield_width_str();
        let affinity = self.affinity.describe();

        tr_replace! {
            "SKILL_SHIELD_NAME";
            affinity, width, name,
        }
    }

    fn description(&self) -> String {
        use Affinities::*;

        let name = self.shield_type_str();
        let potency = ShieldSkill::multi_description(self.multiplier);
        let width = self.shield_width_str();

        let reflect_action = match self.reflect {
            true => tr!("SKILL_SHIELD_REFLECTIVITY_TRUE"),
            false => tr!("SKILL_SHIELD_REFLECTIVITY_FALSE"),
        };

        let affinity = match &self.affinity {
            Specific(elements) => {
                let iter = elements.iter().map(|x| x.describe_adj());
                join_words(iter, "and", Some("only"))
                    .expect("shield of many elements has empty block list")
            }

            AllElements => "all kinds of".to_owned(),
            Magical => "magical".to_owned(),
            Physical => "physical".to_owned(),
        };

        let part1 = tr_replace! {
            "SKILL_SHIELD_DESC";
            width, potency, name, reflect_action, affinity,
        };

        let hits_str = ShieldSkill::hits_to_str(self.hits);
        tr_replace! {
            "SKILL_SHIELD_COMBINE_PARTS";
            part1, hits_str
        }
    }

    fn base_cost(&self) -> u32 {
        todo!()
    }

    fn cast(
        &self,
        _caster: BattlerPtr,
        _targets: Vec<BattlerPtr>,
        _allies: Vec<BattlerPtr>,
        _enemies: Vec<BattlerPtr>,
    ) {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use Element::*;
//
//     #[test]
//     fn test_describe_impenetrable_all_elements() {
//         let skill = ShieldSkill {
//             affinity: ShieldAffinity::AllElements,
//             hits: 1,
//             multiplier: 0.2,
//             reflect: false,
//             plural: false,
//         };
//
//         assert_eq!(skill.name(), "Almighty Shield");
//         assert_eq!(
//             skill.description(),
//             "Casts a sturdy shield that blocks all kinds of damage once."
//         );
//     }
//
//     #[test]
//     fn test_describe_7hit_reflective_magical() {
//         let skill = ShieldSkill {
//             affinity: ShieldAffinity::Magical,
//             hits: 7,
//             multiplier: 0.8,
//             reflect: true,
//             plural: false,
//         };
//
//         assert_eq!(skill.name(), "Magical Mirror");
//         assert_eq!(
//             skill.description(),
//             "Casts a weak shield that reflects magical damage many times."
//         );
//     }
//
//     #[test]
//     fn test_describe_two_specific_elements() {
//         let skill = ShieldSkill {
//             affinity: ShieldAffinity::Specific(vec![Psi, Spirit]),
//             hits: 3,
//             multiplier: 0.5,
//             reflect: false,
//             plural: false,
//         };
//
//         assert_eq!(
//             skill.description(),
//             "Casts a shield that blocks Psychic and Supernatural damage a couple times."
//         );
//     }
//
//     #[test]
//     fn test_describe_many_specific_elements() {
//         let skill = ShieldSkill {
//             affinity: ShieldAffinity::Specific(vec![Fire, Psi, Ice, Spirit]),
//             hits: 3,
//             multiplier: 0.5,
//             reflect: false,
//             plural: false,
//         };
//
//         assert_eq!(
//             skill.description(),
//             indoc! {"Casts a shield that blocks Fire-based, Psychic, \
//             Ice-based, and Supernatural damage a couple times."}
//         );
//     }
//
//     #[test]
//     fn test_describe_wide_one_specific_element() {
//         let skill = ShieldSkill {
//             affinity: ShieldAffinity::Specific(vec![Fuzz]),
//             hits: 3,
//             multiplier: 0.5,
//             reflect: false,
//             plural: true,
//         };
//
//         assert_eq!(skill.name(), "Specialized Wide Shield");
//         assert_eq!(
//             skill.description(),
//             "Casts a wide shield that blocks only Fuzzy damage a couple times."
//         );
//     }
//
//     #[test]
//     fn test_describe_physical_as_specific() {
//         let mut skill = ShieldSkill {
//             affinity: ShieldAffinity::Magical,
//             hits: 3,
//             multiplier: 0.5,
//             reflect: false,
//             plural: true,
//         };
//
//         assert_eq!(skill.name(), "Magical Wide Shield");
//
//         let new_aff = ShieldAffinity::Specific(Element::list_physical());
//         skill.set_affinity(new_aff);
//
//         assert_eq!(skill.name(), "Physical Wide Shield");
//         assert_eq!(
//             skill.description(),
//             "Casts a wide shield that blocks physical damage a couple times."
//         );
//     }
// }
