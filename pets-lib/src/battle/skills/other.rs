use super::*;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct PSIFluxSkill(pub Duration);

#[typetag::serde]
impl Skill for PSIFluxSkill {
    fn name(&self) -> String {
        tr!("SKILL_PSI_FLUX_NAME").to_string()
    }

    fn description(&self) -> String {
        let time = self.0.as_secs().to_string();
        tr_replace!("SKILL_PSI_FLUX_DESC"; time)
    }

    fn base_cost(&self) -> IntegralStat {
        todo!()
    }

    fn cast(
        &self,
        _caster: Rc<RefCell<Battler>>,
        _target: Rc<RefCell<Battler>>,
        _allies: Vec<Rc<RefCell<Battler>>>,
        _enemies: Vec<Rc<RefCell<Battler>>>,
    ) {
        todo!()
    }
}
