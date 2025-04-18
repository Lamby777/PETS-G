use std::borrow::BorrowMut;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackSkill {
    /// translation key to the skill's name
    pub tr_key: String,

    pub element: Element,
    pub power: Option<u8>,
    pub plural: bool,
    pub status_effect: Option<EffectAndChance>,
}

impl AttackSkill {
    #[deprecated]
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
impl Skill for AttackSkill {
    fn name(&self) -> String {
        let family = tr!("{}", self.tr_key.clone());
        let power = self.power.map(|p| power_to_letter_pl(p, self.plural));

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

    fn base_cost(&self) -> IntegralStat {
        1
    }

    fn cast(
        &self,
        caster: Rc<RefCell<Battler>>,
        target: Rc<RefCell<Battler>>,
        _allies: Vec<Rc<RefCell<Battler>>>,
        _enemies: Vec<Rc<RefCell<Battler>>>,
    ) {
        let caster = RefCell::borrow(&caster);
        let mut target = RefCell::borrow_mut(&target);

        let attack = caster.practical_stats().attack;

        if let Some(power) = self.power {
            let damage = attack * power as IntegralStat;
            godot_print!(
                "Dealing {} damage, from {} attack * {} power",
                damage,
                attack,
                power
            );
            target.take_damage(damage);

            godot_print!("Target HP after damage: {}", target.battle_stats.hp);
        }

        if let Some(effect) = &self.status_effect {
            if !effect.chance.roll() {
                godot_print!("Status effect rolled false.");
                return;
            }

            godot_print!("Applying status effect: {:?}", effect.effect);
            target.apply_status_effect(effect.effect);
        }
    }
}
