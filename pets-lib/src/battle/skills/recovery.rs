use super::*;

#[derive(Serialize, Deserialize)]
pub struct RecoverySkill {
    pub name: String,
    pub recovery: RecoveryType,
}

#[derive(Serialize, Deserialize)]
pub enum RecoveryType {
    HPPercent { percent: f64 },
    HPAmount { amount: u8 },
    Status { rating: u8 },
}

#[typetag::serde]
impl SkillFamily for RecoverySkill {
    fn name(&self) -> String {
        self.name.clone()
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

    fn base_cost(&self) -> u32 {
        todo!()
    }

    fn cast(
        &self,
        _caster: Rc<RefCell<dyn Battler>>,
        _targets: Vec<Rc<RefCell<dyn Battler>>>,
        _allies: Vec<Rc<RefCell<dyn Battler>>>,
        _enemies: Vec<Rc<RefCell<dyn Battler>>>,
    ) {
        todo!()
    }
}
