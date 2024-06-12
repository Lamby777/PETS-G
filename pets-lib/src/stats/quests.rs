use godot::prelude::*;
// use crate::prelude::*;

pub type QuestPhase = i32;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Quest {
    base: Base<Node>,

    #[export]
    #[init(default = 0)]
    pub phase: QuestPhase,

    #[export]
    #[init(default = 1)]
    final_phase: QuestPhase,
    // #[export]
    // #[init(default = false)]
    // pub active: bool,
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

    #[func]
    pub fn quest_name(&self) -> GString {
        self.base().get_name().into()
    }
}
