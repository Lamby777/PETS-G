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
                format!("Heals {} HP.", amount)
            }

            HPPercent { percent, .. } => {
                format!("Heals {}% of the target's HP.", percent)
            }

            Status { rating, .. } => {
                format!("Heals status effects up to {}★.", rating)
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
        _target: Option<BattlerPtr>,
        _allies: Vec<BattlerPtr>,
        _enemies: Vec<BattlerPtr>,
    ) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_heal_20_percent() {
        let skill = RecoverySkill {
            name: "Deez".to_owned(),
            recovery: RecoveryType::HPPercent { percent: 20.0 },
        };
        assert_eq!(skill.description(), "Heals 20% of the target's HP.");
    }

    #[test]
    fn test_describe_heal_50_hp() {
        let skill = RecoverySkill {
            name: "Deez".to_owned(),
            recovery: RecoveryType::HPAmount { amount: 50 },
        };
        assert_eq!(skill.description(), "Heals 50 HP.");
    }

    #[test]
    fn test_describe_heal_status_3star() {
        let skill = RecoverySkill {
            name: "Deez".to_owned(),
            recovery: RecoveryType::Status { rating: 3 },
        };
        assert_eq!(skill.description(), "Heals status effects up to 3★.");
    }
}
