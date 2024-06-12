//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::prelude::*;

use super::charmap::default_charmap;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    base: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    pub save: SaveFile,

    /// Hash map of function pointers for calculating stats
    statcalcs: CharStatCalcs,
}

#[godot_api]
impl StatsInterface {
    pub fn load_save_state(&mut self, save: SaveFile) {
        self.save = save;
    }

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
        let (_, statcalcs) = default_charmap();
        let save = SaveFile::fresh();

        load_item_registry();

        Self {
            base,
            save,
            statcalcs,
        }
    }
}
