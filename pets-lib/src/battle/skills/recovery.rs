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

impl RecoveryType {
    fn describe(&self) -> String {
        use RecoveryType::*;

        match self {
            HPAmount { amount, .. } => {
                let template = tr!("SKILL_RECOVERY_HP_AMOUNT_DESC");
                template
                    .to_string()
                    .replace("{amount}", &amount.to_string())
            }

            HPPercent { percent, .. } => {
                let template = tr!("SKILL_RECOVERY_HP_PERCENT_DESC");
                template
                    .to_string()
                    .replace("{percent}", &percent.to_string())
            }

            Status { rating, .. } => {
                let template = tr!("SKILL_RECOVERY_STATUS_DESC");
                template
                    .to_string()
                    .replace("{rating}", &rating.to_string())
            }
        }
    }
}

#[typetag::serde]
impl SkillFamily for RecoverySkill {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.recovery.describe()
    }

    fn base_cost(&self) -> u32 {
        todo!()
    }

    fn cast(
        &self,
        _caster: BattlerPtr,
        _targets: Vec<BattlerPtr>,
        _allies: Vec<BattlerPtr>,
        _enemies: Vec<BattlerPtr>,
    ) {
        todo!()
    }
}
