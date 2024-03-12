use super::*;

#[derive(Serialize, Deserialize)]
pub struct AttackSkill {
    pub element: Element,
    pub power: u8,
    pub plural: bool,
    pub status_effect: Option<ChanceOfEffect>,
}

impl AttackSkill {
    pub fn new(element: Element, power: u8) -> Self {
        Self {
            element,
            power,
            plural: false,
            status_effect: None,
        }
    }

    pub fn with_effect(mut self, effect: StatusEffect, chance: EffectChance) -> Self {
        self.status_effect = Some(ChanceOfEffect::new(effect, chance));
        self
    }

    pub fn make_plural(mut self) -> Self {
        self.plural = true;
        self
    }

    fn describe_power(&self) -> Option<&str> {
        Some(match self.power {
            0 => return None,
            1 => "faint",
            2 => "weak",
            3 => "medium",
            4 => "strong",
            5 => "massive",
            _ => unreachable!(),
        })
    }

    fn describe_damage(&self) -> Option<String> {
        self.describe_power().map(|power| {
            let element = self.element.describe();
            format!("Deals {} {} damage.", power, element)
        })
    }
}

#[typetag::serde]
impl SkillFamily for AttackSkill {
    fn name(&self) -> String {
        todo!()
    }

    /// Panics if neither damage nor effect are present
    fn description(&self) -> String {
        let dmg = self.describe_damage();
        let fx = self.status_effect.as_ref().map(|fx| fx.describe());

        let p1 = match (dmg, fx) {
            (Some(dmg), Some(fx)) => format!("{} {}", dmg, fx),
            (Some(dmg), None) => dmg,
            (None, Some(fx)) => fx,
            (None, None) => panic!("no damage or effect to format"),
        };

        if self.plural {
            format!("{} Targets all enemies!", p1)
        } else {
            p1
        }
    }

    fn base_cost(&self) -> u32 {
        1
    }

    fn cast(
        &self,
        _caster: CharStatsPtr,
        target: Option<CharStatsPtr>,
        _allies: Vec<CharStatsPtr>,
        _enemies: Vec<CharStatsPtr>,
    ) {
        let target = target.expect("attack skill should have a target");
        // let damage = self.power as u32;
        // let mut target = target.borrow_mut();
        // target.hp = target.hp.saturating_sub(damage);
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_dmg_low_chance_effect() {
        let skill = AttackSkill::new(Element::Fire, 3)
            .with_effect(StatusEffect::Burning, EffectChance::Rare);

        assert_eq!(
            skill.description(),
            "Deals medium Fire-based damage. Low chance of inflicting On Fire."
        );
    }

    #[test]
    fn test_describe_dmg() {
        let skill = AttackSkill::new(Element::Fire, 1);

        assert_eq!(skill.description(), "Deals faint Fire-based damage.");
    }

    #[test]
    fn test_describe_low_chance_effect() {
        let skill = AttackSkill::new(Element::Fire, 0)
            .with_effect(StatusEffect::Burning, EffectChance::Common);

        assert_eq!(skill.description(), "High chance of inflicting On Fire.");
    }

    #[test]
    fn test_describe_dmg_nonbased() {
        let skill = AttackSkill::new(Element::Psi, 4);

        assert_eq!(skill.description(), "Deals strong Psychic damage.");
    }

    #[test]
    fn test_describe_molotov_cocktail() {
        let skill = AttackSkill::new(Element::Fire, 0)
            .with_effect(StatusEffect::Burning, EffectChance::Guaranteed)
            .make_plural();

        assert_eq!(
            skill.description(),
            "Always inflicts On Fire. Targets all enemies!"
        );
    }
}
