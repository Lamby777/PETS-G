use super::*;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct PSIFluxSkill(pub Duration);

#[typetag::serde]
impl SkillFamily for PSIFluxSkill {
    fn name(&self) -> String {
        tr!("SKILL_PSI_FLUX_NAME").to_string()
    }

    fn description(&self) -> String {
        let time = self.0.as_secs().to_string();
        let template = tr!("SKILL_PSI_FLUX_DESC");

        template.to_string().replace("{seconds}", &time)
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
    fn name(&self) -> String {
        tr!("SKILL_PSI_REWIRE_NAME").to_string()
    }

    fn description(&self) -> String {
        let percent = self.multi_as_percent_str();
        let template = tr!("SKILL_PSI_REWIRE_DESC");

        template.to_string().replace("{percent}", &percent)
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
