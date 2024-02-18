use super::*;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct PSIFluxSkill(pub Duration);

#[typetag::serde]
impl SkillFamily for PSIFluxSkill {
    fn description(&self) -> String {
        format!("Warps time in your favor for {} seconds.", self.0.as_secs())
    }
}

#[derive(Serialize, Deserialize)]
pub struct PSIRewireSkill {
    /// The cut of mana you put in (float between 0.0 and 1.0)
    ///
    /// Doesn't make you any "more lucky," but if you're desperate
    /// enough to use this skill, you'll probably only use it once
    /// anyway, so it gives you better returns if you win... while
    /// also screwing you over harder if you lose. Not an issue if
    /// you've got nothing to lose, right?
    ///
    /// Search up the "law of large numbers" or "gambler's ruin"
    /// if you're curious. :)
    pub multi: f64,
}

impl PSIRewireSkill {
    // up to (31% lower | 30% higher) your staked mana
    const MARGINS: (f64, f64) = (0.69, 1.30);

    pub fn roll(&self, input: IntegralStat, cap: IntegralStat) -> IntegralStat {
        // roll between low and high
        let mut rng = rand::thread_rng();
        let mult: f64 = rng.gen_range(Self::MARGINS.0..=Self::MARGINS.1);

        // what kind of insane floating-point witchcraft is this?
        (input as f64 * mult).floor().min(cap as f64).max(0.0) as IntegralStat
    }

    pub fn multi_as_percent_str(&self) -> String {
        format!("{}%", self.multi * 100.0)
    }
}

#[typetag::serde]
impl SkillFamily for PSIRewireSkill {
    fn description(&self) -> String {
        format!(
            "Gamble away {} of your mana for the rare chance of a profit.",
            self.multi_as_percent_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_rewire_10_sec() {
        let skill = PSIFluxSkill(Duration::from_secs(10));
        assert_eq!(
            skill.description(),
            "Warps time in your favor for 10 seconds."
        );
    }

    #[test]
    fn test_describe_rewire_50_percent() {
        let skill = PSIRewireSkill { multi: 0.5 };
        assert_eq!(
            skill.description(),
            "Gamble away 50% of your mana for the rare chance of a profit."
        );
    }
}
