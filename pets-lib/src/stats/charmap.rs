use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

use super::statcalc::{CharStatCalcs, StatCalcFn, StatCalcList};

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    let mut res = CharMap::new();

    // add chars from registry
    for chname in PChar::ALL.iter() {
        res.insert(
            chname.to_string(),
            Rc::new(RefCell::new(CharData::default())),
        );
    }

    res
}

/// The default stat calculation functions for all characters
pub fn uniform_statcalcmap() -> CharStatCalcs {
    let mut res = CharStatCalcs::new();

    for chname in PChar::ALL.iter() {
        res.insert(chname.to_string(), StatCalcList::default());
    }

    res
}

/// "Jat Chippity goes hard"
/// Makes it easier to write custom base stats and stuff
macro_rules! ch_unique {
    ($map:expr, $calcs:expr, $($character:ident {
        $($field:ident $(.$property:ident)? = $value:expr,)*
        $(;$base:ident => $base_fn:expr),*
    }),*) => {
        $(
            let character = PChar::$character;
            $map.entry(character.to_owned()).and_modify(|pchar| {
                let mut pchar = pchar.borrow_mut();
                $(
                    pchar.$field$(.$property)? = $value;
                )*
            });

            $calcs.insert(character.to_owned(), {
                let calcs = StatCalcList {
                    $(
                        $base: StatCalcFn::from($base_fn as fn(_) -> _),
                    )*
                    ..Default::default()
                };

                calcs
            });
        )*
    };
}

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> (CharMap, CharStatCalcs) {
    let mut res_map = uniform_charmap();
    let mut res_calcs = uniform_statcalcmap();

    // max_hp, max_energy, attack, defense, speed,
    // stability, delta, epsilon, lambda, max_mana,

    ch_unique! {
        res_map,
        res_calcs,

        ETHAN {
            display_name = "Ethan".to_string(),

            ;max_hp => |lvl| lvl - 6,
            ;max_mana => |lvl| Some(lvl + 1)
        },

        SIVA {
            display_name = "Siva".to_string(),

            ;max_hp => |lvl| lvl - 2,
            ;max_mana => |lvl| Some(lvl + 1)
        },

        TERRA {
            display_name = "Terra".to_string(),

            ;max_hp => |lvl| lvl + 6
        }
    }

    (res_map, res_calcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "broken test, update when chardata is actually in use"]
    #[test]
    fn ch_unique_charmap() {
        let (charmap, _) = default_charmap();

        let ethan = charmap.get(&PChar::ETHAN.to_string()).unwrap();
        let ethan = ethan.borrow();
        assert_eq!(ethan.display_name, "Ethan");
        // assert_eq!(ethan.base_stats.max_hp, 12);
        // assert_eq!(ethan.base_stats.lambda, Some(1));
        // assert_eq!(ethan.base_stats.max_mana, Some(1));
    }

    #[test]
    fn ch_unique_calcs() {
        let (_, calcs) = default_charmap();

        let ethan = calcs.get(&PChar::ETHAN.to_string()).unwrap();
        let energy_fn = *ethan.max_energy;
        assert_eq!(energy_fn(10), 11);
    }
}
