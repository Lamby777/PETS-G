//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::global::randomize;
use godot::prelude::*;

use crate::battle::skills::load_skill_registry;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    base: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    pub save: SaveFile,
}

#[godot_api]
impl StatsInterface {
    pub fn load_save_state(&mut self, save: SaveFile) {
        self.save = save;
    }

    // #[func]
    pub fn get_character(&self, ch: &PChar) -> CharData {
        self.save
            .chars
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
            .take()
    }

    #[func]
    pub fn get_quest_phase(&self, quest_id: GString) -> QuestPhase {
        *self.save.quests.get(&quest_id.to_string()).unwrap_or(&-1)
    }

    #[func]
    pub fn set_quest_phase(&mut self, quest_id: GString, phase: QuestPhase) {
        self.save.quests.insert(quest_id.to_string(), phase);
    }
}

impl GodotAutoload for StatsInterface {
    const AUTOLOAD_NAME: &'static str = "Stats";
}

#[godot_api]
impl IObject for StatsInterface {
    fn init(base: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let save = SaveFile::fresh();

        load_item_registry();
        load_skill_registry();

        // randomize seed for godot
        randomize();

        print_debug_crap();

        Self { base, save }
    }
}

/// put stuff like serialization testing in here temporarily to mess around
/// it'll run when the game starts
fn print_debug_crap() {
    let ser = crate::stats::charmap::default_charmap();
    let ser = toml::to_string(&ser).unwrap();
    godot_print!("{}", ser);
}
