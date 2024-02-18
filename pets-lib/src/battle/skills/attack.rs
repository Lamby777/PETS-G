use super::*;

pub struct SkillAttack {
    pub name: &'static str,
    pub cost: u32,
    pub power: u8,
    pub element: Element,
    pub status_effect: Option<EffectPair>,
}

impl SkillAttack {
    fn describe_power(&self) -> Option<&str> {
        Some(match self.power {
            0 => return None,
            1 => "Faint",
            2 => "Weak",
            3 => "Medium",
            4 => "Strong",
            5 => "Massive",
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
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> String {
        let dmg = self.describe_damage();
        let fx = self.describe_effects();

        //
    }

    fn cost(&self) -> u32 {
        self.cost
    }
}
