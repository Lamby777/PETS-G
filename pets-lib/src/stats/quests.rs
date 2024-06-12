use godot::prelude::*;

use crate::prelude::*;

pub type QuestPhase = u32;

/// Map of the quest ID -> its phase
/// For saving purposes.
#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct Quests(HashMap<String, QuestPhase>);

impl Quests {
    pub fn fresh() -> Self {
        Self(HashMap::new())
    }
}

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
    pub fn final_phase(&self) -> QuestPhase {
        self.final_phase
    }

    pub fn is_complete(&self) -> bool {
        self.phase >= self.final_phase
    }
}
