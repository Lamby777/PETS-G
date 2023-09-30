use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

use super::statcalc::{CharStatCalcs, StatCalcList};

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
    let default = Rc::new(StatCalcList::default());

    for chname in PChar::ALL.iter() {
        let cloned = default.clone();
        res.insert(chname.to_string(), cloned);
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
                        $base: $base_fn,
                    )*
                    ..Default::default()
                };

                Rc::new(calcs)
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
            base_stats.max_hp = 12,
            base_stats.max_mana = Some(1),
            base_stats.lambda = Some(1),
        },

        SIVA {
            display_name = "Siva".to_string(),
            base_stats.max_hp = 18,
            base_stats.max_mana = Some(1),
        },

        TERRA {
            display_name = "Terra".to_string(),
            base_stats.max_hp = 26,
        }
    }

    (res_map, res_calcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch_unique_macro_works() {
        let (charmap, _statcalcs) = default_charmap();

        let ethan = charmap.get(&PChar::ETHAN.to_string()).unwrap();
        let ethan = ethan.borrow();
        assert_eq!(ethan.display_name, "Ethan");
        assert_eq!(ethan.base_stats.max_hp, 12);
        assert_eq!(ethan.base_stats.lambda, Some(1));
        assert_eq!(ethan.base_stats.max_mana, Some(1));

        let siva = charmap.get(&PChar::SIVA.to_string()).unwrap();
        let siva = siva.borrow();
        assert_eq!(siva.display_name, "Siva");
        assert_eq!(siva.base_stats.max_hp, 18);
        assert_eq!(siva.base_stats.lambda, None);
        assert_eq!(siva.base_stats.max_mana, Some(1));
    }
}
