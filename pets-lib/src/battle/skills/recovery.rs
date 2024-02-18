use super::*;

#[derive(Serialize, Deserialize)]
pub enum RecoverySkill {
    HPPercent(f64),
    HPAmount(u8),
    Status { rating: u8 },
}

#[typetag::serde]
impl SkillFamily for RecoverySkill {
    fn description(&self) -> String {
        use RecoverySkill::*;

        match self {
            HPPercent(power) => format!("Heals {}% of the target's HP.", power),
            HPAmount(power) => format!("Heals {} HP.", power),
            Status { rating } => {
                format!("Heals status conditions up to {}★.", rating)
            }
        }
    }

    fn base_cost(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_heal_20_percent() {
        let skill = RecoverySkill::HPPercent(20.0);
        assert_eq!(skill.description(), "Heals 20% of the target's HP.");
    }

    #[test]
    fn test_describe_heal_50_hp() {
        let skill = RecoverySkill::HPAmount(50);
        assert_eq!(skill.description(), "Heals 50 HP.");
    }

    #[test]
    fn test_describe_heal_status_3star() {
        let skill = RecoverySkill::Status { rating: 3 };
        assert_eq!(skill.description(), "Heals status conditions up to 3★.");
    }
}
