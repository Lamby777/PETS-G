use super::*;

pub struct SkillAttack {
    pub element: Element,
    pub power: u8,
    pub status_effect: Option<ChanceOfEffect>,
}

impl SkillAttack {
    pub fn new(element: Element, power: u8) -> Self {
        Self {
            element,
            power,
            status_effect: None,
        }
    }

    pub fn with_effect(mut self, effect: StatusEffect, chance: EffectChance) -> Self {
        self.status_effect = Some(ChanceOfEffect::new(effect, chance));
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

impl Skill for SkillAttack {
    fn description(&self) -> String {
        let mut res = self.describe_damage().unwrap_or_default();

        if let Some(fx) = &self.status_effect {
            format!("{} {}", res, fx.describe())
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmg_low_chance_effect_description() {
        let skill = SkillAttack::new(Element::Fire, 3)
            .with_effect(StatusEffect::Heatstroke, EffectChance::Rare);

        assert_eq!(
            skill.description(),
            "Deals medium Fire-based damage. Low chance of inflicting Heatstroke."
        );
    }

    #[test]
    fn test_low_chance_effect_description() {
        let skill = SkillAttack::new(Element::Fire, 0)
            .with_effect(StatusEffect::Heatstroke, EffectChance::Common);

        assert_eq!(skill.description(), "High chance of inflicting Heatstroke.");
    }
}
