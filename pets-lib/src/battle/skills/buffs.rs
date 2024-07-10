use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuffSkill {
    pub name: String,
    pub offsets: InherentStats,
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
