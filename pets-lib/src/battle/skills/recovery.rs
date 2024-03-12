use super::*;

#[derive(Serialize, Deserialize)]
pub enum RecoverySkill {
    HPPercent { name: String, percent: f64 },
    HPAmount { name: String, amount: u8 },
    Status { name: String, rating: u8 },
}

#[typetag::serde]
impl SkillFamily for RecoverySkill {
    fn name(&self) -> String {
        use RecoverySkill::*;
        match self {
            HPPercent { name, .. } => name.clone(),
            HPAmount { name, .. } => name.clone(),
            Status { name, .. } => name.clone(),
        }
    }

    fn description(&self) -> String {
        use RecoverySkill::*;

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

    fn base_cost(&self) -> u32 {
        todo!()
    }

    fn cast(
        &self,
        _caster: CharStatsPtr,
        _target: Option<CharStatsPtr>,
        _allies: Vec<CharStatsPtr>,
        _enemies: Vec<CharStatsPtr>,
    ) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_heal_20_percent() {
        let skill = RecoverySkill::HPPercent {
            name: "Deez".to_owned(),
            percent: 20.0,
        };
        assert_eq!(skill.description(), "Heals 20% of the target's HP.");
    }

    #[test]
    fn test_describe_heal_50_hp() {
        let skill = RecoverySkill::HPAmount {
            name: "Deez".to_owned(),
            amount: 50,
        };
        assert_eq!(skill.description(), "Heals 50 HP.");
    }

    #[test]
    fn test_describe_heal_status_3star() {
        let skill = RecoverySkill::Status {
            name: "Deez".to_owned(),
            rating: 3,
        };
        assert_eq!(skill.description(), "Heals status effects up to 3★.");
    }
}
