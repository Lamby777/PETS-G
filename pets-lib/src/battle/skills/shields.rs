use super::*;

/// What kind of damage does the shield block?
#[derive(Serialize, Deserialize)]
pub enum ShieldAffinity {
    Physical,
    Magical,
    AllElements,

    Specific(Vec<Element>),
}

impl ShieldAffinity {
    /// Attempts to convert an explicit list of elements like
    /// [Blade, Kinetic] into a more general variant like Physical
    fn coerce_specific(&self) -> Option<Self> {
        use ShieldAffinity::*;

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

    fn describe(&self) -> &str {
        use ShieldAffinity::*;

        match self {
            Specific(_) => "Specialized",
            AllElements => "Almighty",
            Magical => "Magical",
            Physical => "Physical",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    ///
    /// Use `set_affinity` to set this, not directly.
    /// The setter will handle converting lists of specific
    /// elements into more general affinities where possible.
    affinity: ShieldAffinity,

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
    fn multi_to_str(multi: f64) -> &'static str {
        if multi < 0.0 {
            panic!("shield with negative multiplier");
        }

        if multi == 0.0 {
            "impenetrable "
        } else if multi <= 0.3 {
            "sturdy "
        } else if multi <= 0.7 {
            ""
        } else if multi <= 1.0 {
            "weak "
        } else {
            // if your shield powers have been weakened past 1.0...
            "nullified "
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

    pub fn set_affinity(&mut self, aff: ShieldAffinity) {
        self.affinity = aff.coerce_specific().unwrap_or(aff);
    }
}

#[typetag::serde]
impl SkillFamily for ShieldSkill {
    fn name(&self) -> String {
        let name = if self.reflect { "Mirror" } else { "Shield" };
        let width = if self.plural { "Wide " } else { "" };
        let aff = self.affinity.describe();

        format!("{aff} {width}{name}")
    }

    fn description(&self) -> String {
        use ShieldAffinity::*;

        let potency = ShieldSkill::multi_to_str(self.multiplier);
        let reflectivity = if self.reflect { "reflects" } else { "blocks" };
        let width = if self.plural { "wide " } else { "" };
        let affinity = match &self.affinity {
            Specific(elements) => {
                let iter = elements.iter().map(|x| x.describe());
                join_words(iter, "and", Some("only"))
                    .expect("shield of many elements has empty block list")
            }

            AllElements => "all kinds of".to_owned(),
            Magical => "magical".to_owned(),
            Physical => "physical".to_owned(),
        };

        let part1 = format!(
            "Casts a {}{}shield that {} {} damage",
            width, potency, reflectivity, affinity
        );

        match self.hits {
            0 => format!("{}. It probably won't last...", potency),
            hits => {
                let hits = ShieldSkill::hits_to_str(hits);
                format!("{} {}.", part1, hits)
            }
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
            plural: false,
        };

        assert_eq!(skill.name(), "Almighty Shield");
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
            plural: false,
        };

        assert_eq!(skill.name(), "Magical Mirror");
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
            plural: false,
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
            plural: false,
        };

        assert_eq!(
            skill.description(),
            indoc! {"Casts a shield that blocks Fire-based, Psychic, \
            Ice-based, and Supernatural damage a couple times."}
        );
    }

    #[test]
    fn test_describe_wide_one_specific_element() {
        let skill = ShieldSkill {
            affinity: ShieldAffinity::Specific(vec![Fuzz]),
            hits: 3,
            multiplier: 0.5,
            reflect: false,
            plural: true,
        };

        assert_eq!(skill.name(), "Specialized Wide Shield");
        assert_eq!(
            skill.description(),
            "Casts a wide shield that blocks only Fuzzy damage a couple times."
        );
    }

    #[test]
    fn test_describe_physical_as_specific() {
        let mut skill = ShieldSkill {
            affinity: ShieldAffinity::Magical,
            hits: 3,
            multiplier: 0.5,
            reflect: false,
            plural: true,
        };

        assert_eq!(skill.name(), "Magical Wide Shield");

        let new_aff = ShieldAffinity::Specific(Element::list_physical());
        skill.set_affinity(new_aff);

        assert_eq!(skill.name(), "Physical Wide Shield");
        assert_eq!(
            skill.description(),
            "Casts a wide shield that blocks physical damage a couple times."
        );
    }
}
