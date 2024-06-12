use crate::prelude::*;

pub type QuestPhase = u32;

#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct Quests(HashMap<String, Quest>);

impl Quests {
    pub fn fresh() -> Self {
        let mut this = HashMap::new();
        let quest = Quest::new(12);
        this.insert("MAIN_STORY".to_string(), quest);

        Self(this)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quest {
    pub phase: QuestPhase,
    final_phase: QuestPhase,
}

impl Quest {
    pub fn new(final_phase: QuestPhase) -> Self {
        Self {
            phase: 0,
            final_phase,
        }
    }

    pub fn final_phase(&self) -> QuestPhase {
        self.final_phase
    }

    pub fn is_complete(&self) -> bool {
        self.phase >= self.final_phase
    }
}
