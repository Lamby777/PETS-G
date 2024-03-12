use super::*;

#[derive(Serialize, Deserialize)]
pub struct BuffSkill {
    pub name: String,
    pub offsets: InherentStats,
    pub turns: u8,
}

#[typetag::serde]
impl SkillFamily for BuffSkill {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        todo!()
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
