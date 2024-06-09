use super::*;

#[derive(Serialize, Deserialize)]
pub struct AttackSkill {
    /// translation key to the skill's name
    pub tr_key: String,

    pub element: Element,
    pub power: u8,
    pub plural: bool,
    pub status_effect: Option<EffectAndChance>,
}

impl AttackSkill {
    pub fn new(tr_key: &str, element: Element, power: u8) -> Self {
        Self {
            tr_key: tr_key.to_owned(),
            element,
            power,
            plural: false,
            status_effect: None,
        }
    }

    pub fn with_effect(
        mut self,
        effect: StatusEffect,
        chance: EffectChance,
    ) -> Self {
        self.status_effect = Some(EffectAndChance::new(effect, chance));
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
        tr!("{}", self.tr_key.clone()).to_string()
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
            tr!("{part1} Targets all enemies!", part1 = p1).to_string()
        } else {
            p1
        }
    }

    fn base_cost(&self) -> u32 {
        1
    }

    fn cast(
        &self,
        _caster: BattlerPtr,
        targets: Vec<BattlerPtr>,
        _allies: Vec<BattlerPtr>,
        _enemies: Vec<BattlerPtr>,
    ) {
        if targets.is_empty() {
            panic!("attack skill should have a target");
        }

        todo!();
    }
}
