use super::*;

pub struct SkillAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub cost: u32,
    pub power: u8,
    pub element: Element,
    pub status_effect: Option<EffectPair>,
}

impl Skill for SkillAttack {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn cost(&self) -> u32 {
        self.cost
    }
}
