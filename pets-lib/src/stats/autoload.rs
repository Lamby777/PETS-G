//!
//! Singleton for accessing player stats in GDScript.
//!

use std::io::Read;

use godot::engine::file_access::ModeFlags;
use godot::prelude::*;

use super::charmap::default_charmap;
use crate::prelude::*;

/// # Memory
///
///  This function leaks memory. It only runs once, and also has a
///  100MB cap on the size of the file it reads, so it shouldn't be
///  a big deal. I just typically put a warning label on any function
///  that leaks memory, so here it is. You've been warned.
pub fn find_modded_item_paths() -> Option<Vec<&'static str>> {
    let mut file = GFile::open("user://mod_items.txt", ModeFlags::READ).ok()?;

    let mut content = vec![];
    file.read_to_end(&mut content).ok()?;
    if content.len() > 100_000_000 {
        godot_warn!("mod_items.txt is too large! (over 100MB, wtf?)");
        godot_warn!("None of your modded items will be loaded!");
        return None;
    }

    let content = String::from_utf8(content).ok()?;

    let paths = content
        .lines()
        .map(|v| &*String::leak(v.to_owned()))
        .collect();
    Some(paths)
}

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    base: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    save: SaveFile,

    /// Hash map of function pointers for calculating stats
    statcalcs: CharStatCalcs,
}

#[godot_api]
impl StatsInterface {
    // #[func]
    pub fn get_character(&self, ch: &str) -> CharData {
        self.save
            .chars
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
            .take()
    }

    /// Get the list of stat calculation functions for a given character
    pub fn get_statcalc(&self, ch: &str) -> Rc<StatCalcList> {
        self.statcalcs
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

/// name the function `x_of`, where `x` is the stat name
/// for example, `si.natural_speed_of(PChar::ETHAN)`
macro_rules! impl_stat_getters_on_si {
    ($($stat:ident),* $(,)?) => {
        #[allow(unused)]
        impl StatsInterface {$(
            concat_idents::concat_idents!(fn_name = natural_, $stat, _of {
                /// Get the stat of a given character at a level,
                /// not including equips or consumables
                pub fn fn_name(&self, pchar: &str) -> IntegralStat {
                    // get character level
                    let ch = self.get_character(pchar);
                    let lvl = ch.level;

                    // get calculation fn for character
                    let calcs = self.get_statcalc(pchar);

                    // calculate the stat
                    (calcs.speed)(lvl)
                }
            });
        )*}
    };
}

// generate getters for character stats
impl_stat_getters_on_si! {
    max_hp,
    max_energy,
    attack,
    defense,
    speed,
    stability,
    delta,
    epsilon,
    lambda,
    max_mana,
}

impl Autoload for StatsInterface {
    const AUTOLOAD_NAME: &'static str = "Stats";
}

#[godot_api]
impl IObject for StatsInterface {
    fn init(base: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let (charmap, statcalcs) = default_charmap();
        let save = SaveFile { chars: charmap };

        match find_modded_item_paths() {
            Some(v) => load_item_registry(&v),
            None => godot_print!("There was an issue loading modded items..."),
        }

        Self {
            base,
            save,
            statcalcs,
        }
    }
}
