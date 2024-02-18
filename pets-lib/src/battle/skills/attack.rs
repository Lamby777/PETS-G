use super::*;

pub struct SkillAttack {
    pub power: u8,
    pub element: Element,
    pub status_effect: Option<ChanceOfEffect>,
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
    fn description(&self) -> String {
        let mut res = self.describe_damage().unwrap_or_default();

        if let Some(fx) = &self.status_effect {
            res += &fx.describe();
        }

        res
    }
}
