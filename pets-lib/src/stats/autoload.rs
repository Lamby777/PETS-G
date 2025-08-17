//!
//! Singleton for accessing player stats in GDScript.
//!

use std::sync::LazyLock;

use godot::classes::{Sprite2D, Texture2D};
use godot::global::randomize;
use godot::prelude::*;

use crate::common::*;

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
    pub fn get_character(&self, ch: &PChar) -> Rc<RefCell<CharData>> {
        self.save.chars.character(ch)
    }

    #[func]
    pub fn get_quest_phase(&self, quest_id: GString) -> QuestPhase {
        *self.save.quests.get(&quest_id.to_string()).unwrap_or(&-1)
    }

    #[func]
    pub fn set_quest_phase(&mut self, quest_id: GString, phase: QuestPhase) {
        self.save.quests.insert(quest_id.to_string(), phase);
    }

    #[func]
    fn set_ethan_bed_color(color: String) {
        let texture = load::<Texture2D>(&format!(
            "res://assets/textures/builds/furniture/beds/bed_{color}.png"
        ));

        si().bind_mut().save.bed_color = color;

        let mut bed =
            World::room().get_node_as::<Sprite2D>("%EthanBed/Sprite2D");
        bed.callv("set_texture", &varray![texture]);
    }
}

impl GodotAutoload for StatsInterface {
    const AUTOLOAD_NAME: &str = "Stats";
}

#[godot_api]
impl IObject for StatsInterface {
    fn init(base: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let save = SaveFile::fresh();

        // load registries, cuz they're `LazyLock`s
        LazyLock::force(&REGISTRIES);

        // randomize seed for godot
        randomize();

        print_debug_crap();

        Self { base, save }
    }
}

/// put stuff like serialization testing in here temporarily to mess around
/// it'll run when the game starts
fn print_debug_crap() {
    // let ser = crate::stats::charmap::default_charmap();
    // let ser = CharMap::_debugging_charmap();
    // let ser = serde_json::to_string(&ser).unwrap();
    // godot_print!("{}", ser);
}
