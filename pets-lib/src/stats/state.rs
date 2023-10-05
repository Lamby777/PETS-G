//!
//! Singleton for accessing player stats in GDScript.
//!

use godot::engine::Engine;
use godot::prelude::*;

use super::charmap::default_charmap;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct StatsInterface {
    #[base]
    node: Base<Object>,

    /// Hash map of info on all the different characters in the game.
    save: SaveFile,

    /// Hash map of function pointers for calculating stats
    statcalcs: CharStatCalcs,
}

/// name the function `x_of`, where `x` is the stat name
/// for example, `si.natural_speed_of(PChars::ETHAN)`
macro_rules! impl_stat_getters_on_si {
    ($($stat:ident),*) => {
        impl StatsInterface {$(
            concat_idents::concat_idents!(fn_name = natural_, $stat, _of {
                /// Get the stat of a given character at a level,
                /// not including equips or consumables
                pub fn fn_name(&self, pchar: &str) -> IntegralStat {
                    // get character level
                    let ch = self.get_character(pchar);
                    let lv = ch.borrow().level;

                    // get calculation fn for character
                    let calcs = self.get_statcalc(pchar);

                    // calculate the stat
                    (*calcs.speed)(lv)
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
    max_mana
}

#[godot_api]
impl StatsInterface {
    /// Get a shared ref to the singleton to store in other node structs
    pub fn singleton() -> Gd<StatsInterface> {
        Engine::singleton()
            .get_singleton("Stats".into())
            .unwrap()
            .cast()
    }

    // #[func]
    pub fn get_character(&self, ch: &str) -> Rc<RefCell<CharData>> {
        self.save
            .chars
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
    }

    /// Get the list of stat calculation functions for a given character
    pub fn get_statcalc(&self, ch: &str) -> Rc<StatCalcList> {
        self.statcalcs
            .get(ch)
            .expect("key should be a valid PChar name")
            .clone()
    }
}

#[godot_api]
impl ObjectVirtual for StatsInterface {
    fn init(node: Base<Object>) -> Self {
        // start an empty save file, but load other if the player
        // picks a save file instead of "new"
        let (charmap, statcalcs) = default_charmap();
        let save = SaveFile { chars: charmap };

        Self {
            node,
            save,
            statcalcs,
        }
    }
}
