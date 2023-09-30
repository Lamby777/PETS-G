use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    let mut res = CharMap::new();

    // add chars from registry
    for chname in PChar::ALL.iter() {
        let cloned = CharData::default();
        res.insert(chname.to_string(), Rc::new(RefCell::new(cloned)));
    }

    res
}

/// "Jat Chippity goes hard"
/// Makes it easier to write custom base stats and stuff
macro_rules! ch_unique {
    ($map:expr, $($character:ident {$($field:ident $(.$property:ident)? = $value:expr),*}),*) => {
        $(
            let character = PChar::$character;
            $map.entry(character.to_owned()).and_modify(|pchar| {
                let mut pchar = pchar.borrow_mut();
                $(
                    pchar.$field$(.$property)? = $value;
                )*
            });
        )*
    };
}

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> CharMap {
    let mut res = uniform_charmap();

    // max_hp, max_energy, attack, defense, speed,
    // stability, delta, epsilon, lambda, max_mana,

    ch_unique! {
        res,

        ETHAN {
            display_name = "Ethan".to_string(),
            base_stats.max_hp = 12,
            base_stats.lambda = Some(1),
            base_stats.max_mana = Some(1)
        },

        SIVA {
            display_name = "Siva".to_string(),
            base_stats.max_hp = 18,
            base_stats.max_mana = Some(1)
        },

        TERRA {
            display_name = "Terra".to_string(),
            base_stats.max_hp = 26
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch_unique_macro_works() {
        let mut charmap = uniform_charmap();

        ch_unique! {
            charmap,

            ETHAN {
                display_name = "Ethan".to_string(),
                base_stats.max_hp = 12,
                base_stats.lambda = Some(1),
                base_stats.max_mana = Some(1)
            },

            SIVA {
                display_name = "Siva".to_string(),
                base_stats.max_hp = 18,
                base_stats.max_mana = Some(1)
            }
        }

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
