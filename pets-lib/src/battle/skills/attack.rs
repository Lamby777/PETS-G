use super::*;

#[derive(Serialize, Deserialize)]
pub struct AttackSkill {
    /// translation key to the skill's name
    pub tr_key: String,

    pub element: Element,
    pub power: Option<u8>,
    pub plural: bool,
    pub status_effect: Option<EffectAndChance>,
}

impl AttackSkill {
    pub fn new(tr_key: &str, element: Element, power: Option<u8>) -> Self {
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

    fn describe_damage(&self) -> Option<String> {
        let element = self.element.adjective();

        // i love rust ^w^
        if let Some(0 | 6..) = self.power {
            panic!("power should be `Some(1..=5)` or `None`");
        }

        let adjective = tr(&format!("SKILL_ATTACK_POWER_{}", self.power?));

        Some(tr_replace! {
            "SKILL_ATTACK_POWER_DESCRIBE";
            adjective, element
        })
    }
}

#[typetag::serde]
impl SkillFamily for AttackSkill {
    fn name(&self) -> String {
        let family = tr!("{}", self.tr_key.clone());
        let power = self.power.map(|p| tr(&format!("SKILL_ATTACK_TIER_{}", p)));

        match power {
            Some(power) => tr_replace! {
                "SKILL_ATTACK_NAME_COMBINED";
                family, power
            },

            None => family.to_string(),
        }
    }

    /// Panics if neither damage nor effect are present
    fn description(&self) -> String {
        let dmg = self.describe_damage();
        let fx = self.status_effect.as_ref().map(|fx| fx.describe());

        let part1 = match (dmg, fx) {
            // Combine both damage and effect descriptions
            (Some(dmg), Some(fx)) => tr_replace! {
                "SKILL_ATTACK_DESCRIBE_COMBINED";
                dmg, fx
            }
            .into(),

            // Use whichever is present
            (Some(dmg), None) => dmg.into(),
            (None, Some(fx)) => fx,

            // Can't have an attack that does 0 damage and no effect
            (None, None) => panic!("no damage or effect to format"),
        };

        if self.plural {
            tr_replace! {
                "SKILL_ATTACK_PLURAL_DESC";
                part1
            }
        } else {
            part1.to_string()
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
