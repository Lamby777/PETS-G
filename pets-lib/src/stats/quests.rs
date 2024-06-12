use crate::prelude::*;

#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct Quests(HashMap<String, Quest>);

impl Quests {
    pub fn fresh() -> Self {
        let mut this = HashMap::new();
        let quest = Quest::default();
        this.insert("INTRO1".to_string(), quest);

        Self(this)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quest {
    pub phase: u8,
}

impl Default for Quest {
    fn default() -> Self {
        Self { phase: 0 }
    }
}
