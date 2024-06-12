use godot::prelude::*;
// use crate::prelude::*;

pub type QuestPhase = i32;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Quest {
    base: Base<Resource>,

    #[export]
    quest_id: GString,

    #[export]
    #[init(default = 0)]
    pub phase: QuestPhase,

    #[export]
    #[init(default = 1)]
    final_phase: QuestPhase,
}

#[godot_api]
impl Quest {
    #[func]
    pub fn final_phase(&self) -> QuestPhase {
        self.final_phase
    }

    #[func]
    pub fn is_complete(&self) -> bool {
        self.phase >= self.final_phase
    }
}
