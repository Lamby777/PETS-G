use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverySkill {
    pub power: u8,
    pub recovery: RecoveryType,
    pub plural: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RecoveryType {
    HPPercent { percent: f64 },
    HPAmount { amount: u8 },
    Status { rating: u8 },
}

#[typetag::serde]
impl Skill for RecoverySkill {
    fn name(&self) -> String {
        let family = match self.recovery {
            RecoveryType::HPAmount { .. } => "SKILL_RECOVERY_HP_AMOUNT",
            RecoveryType::HPPercent { .. } => "SKILL_RECOVERY_HP_PERCENT",
            RecoveryType::Status { .. } => "SKILL_RECOVERY_STATUS",
        };

        let power = power_to_letter_pl(self.power, self.plural);

        tr_replace!("SKILL_RECOVERY_COMBINED"; family, power)
    }

    fn description(&self) -> String {
        use RecoveryType::*;

        match self.recovery {
            HPAmount { amount, .. } => {
                tr_replace!("SKILL_RECOVERY_HP_AMOUNT_DESC"; amount)
            }

            HPPercent { percent, .. } => {
                tr_replace!("SKILL_RECOVERY_HP_PERCENT_DESC"; percent)
            }

            Status { rating, .. } => {
                tr_replace!("SKILL_RECOVERY_STATUS_DESC"; rating)
            }
        }
    }

    fn base_cost(&self) -> IntegralStat {
        todo!()
    }

    fn cast(
        &self,
        _caster: Rc<RefCell<dyn Battler>>,
        _target: Rc<RefCell<dyn Battler>>,
        _allies: Vec<Rc<RefCell<dyn Battler>>>,
        _enemies: Vec<Rc<RefCell<dyn Battler>>>,
    ) {
        todo!()
    }
}
