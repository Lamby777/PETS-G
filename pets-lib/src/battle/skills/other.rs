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
        tr_replace!("SKILL_PSI_FLUX_DESC"; time)
    }

    fn base_cost(&self) -> u32 {
        todo!()
    }

    fn cast(
        &self,
        _caster: Rc<RefCell<dyn Battler>>,
        _targets: Vec<Rc<RefCell<dyn Battler>>>,
        _allies: Vec<Rc<RefCell<dyn Battler>>>,
        _enemies: Vec<Rc<RefCell<dyn Battler>>>,
    ) {
        todo!()
    }
}
