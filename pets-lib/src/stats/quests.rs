use crate::prelude::*;
use godot::prelude::*;

pub type QuestPhase = i32;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Quest {
    base: Base<Node>,

    #[export]
    #[init(val = 0)]
    pub phase: QuestPhase,

    #[export]
    #[init(val = 1)]
    final_phase: QuestPhase,
}

#[godot_api]
impl Quest {
    #[func]
    fn dbox(&self) -> Gd<DialogBox> {
        DialogBox::singleton()
    }

    #[func]
    fn pcb(&self) -> Gd<PlayerCB> {
        pcb()
    }

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
