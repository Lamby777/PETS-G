use crate::prelude::*;

pub type CharMap = HashMap<String, RefCell<CharData>>;

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    PChar::ALL.iter().fold(CharMap::new(), |mut map, chname| {
        map.insert(chname.to_string(), RefCell::new(CharData::default()));
        map
    })
}

/// The default stat calculation functions for all characters
pub fn uniform_statcalcmap() -> CharStatCalcs {
    PChar::ALL
        .iter()
        .fold(CharStatCalcs::new(), |mut calcs, chname| {
            calcs.insert(chname.to_string(), Rc::new(StatCalcList::default()));
            calcs
        })
}

/// "Jat Chippity goes hard"
/// Makes it easier to write custom base stats and stuff
/// Registry of characters with unique stat calculation functions
macro_rules! ch_unique_registry {
    ($map:expr, $calcs:expr, $($character:ident {
        $($field:ident $(.$property:ident)? = $value:expr,)*
        $(;$base:ident => $base_fn:expr),*
    }),* $(,)?) => {
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

    ch_unique_registry! {
        res_map,
        res_calcs,

        ETHAN {
            display_name = "Ethan".to_owned(),

            ;max_hp => |lvl| lvl - 6,
            ;max_mana => |lvl| Some(lvl + 1),
            ;speed => |_| 400
        },

        SIVA {
            display_name = "Siva".to_owned(),

            ;max_hp => |lvl| lvl - 2,
            ;max_mana => |lvl| Some(lvl + 1)
        },

        TERRA {
            display_name = "Terra".to_owned(),

            ;max_hp => |lvl| lvl + 6
        },
    }

    (res_map, res_calcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "broken test"] // update when chardata is actually in use
    #[test]
    fn ch_unique_charmap_works() {
        let (charmap, _) = default_charmap();
        let ethan = charmap.get(PChar::ETHAN).unwrap();
        let ethan = ethan.borrow();
        assert_eq!(ethan.display_name, "Ethan");
        // assert_eq!(ethan.base_stats.max_hp, 12);
        // assert_eq!(ethan.base_stats.lambda, Some(1));
        // assert_eq!(ethan.base_stats.max_mana, Some(1));
    }

    #[test]
    fn ch_unique_calcs_works() {
        let mut charmap = uniform_charmap();
        let mut calcs = uniform_statcalcmap();

        ch_unique_registry! {
            charmap,
            calcs,

            ETHAN {
                display_name = "Ethan".to_owned(),

                ;max_hp => |lvl| lvl - 6,
                ;max_mana => |lvl| Some(lvl + 1),
                ;speed => |_| 400
            },
        }

        let calcs = calcs.get(PChar::ETHAN).unwrap();
        assert_eq!((calcs.max_hp)(20), 14);
        assert_eq!((calcs.max_mana)(40), Some(41));
    }
}
