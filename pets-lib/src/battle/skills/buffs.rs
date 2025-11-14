use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuffSkill {
    pub name: String,
    pub offsets: LeveledStats,
    pub turns: u8,
}

#[typetag::serde]
impl Skill for BuffSkill {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn base_cost(&self) -> IntegralStat {
        todo!()
    }

    fn cast(&self, engine: &mut BattleEngine) {
        todo!()
    }
}
