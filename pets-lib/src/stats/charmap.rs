use strum::IntoEnumIterator as _;

use crate::prelude::*;

pub type CharMap = HashMap<PChar, RefCell<CharData>>;

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    PChar::iter().fold(CharMap::new(), |mut map, chname| {
        map.insert(
            chname,
            RefCell::new(CharData {
                id: chname,
                ..Default::default()
            }),
        );
        map
    })
}

/// "Jat Chippity goes hard"
/// Makes it easier to write custom base stats and stuff
/// Registry of characters with unique stat calculation functions
macro_rules! ch_unique_registry {
    ($map:expr, $($character:ident {
        $($field:ident $(.$property:ident)? = $value:expr,)*
    }),* $(,)?) => {
        $(
            let character = PChar::$character;
            $map.entry(character).and_modify(|pchar| {
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
    let mut res_map = uniform_charmap();

    // max_hp, max_energy, attack, defense, speed,
    // stability, delta, epsilon, lambda, max_mana,

    ch_unique_registry! {
        res_map,

        ETHAN {
            display_name = "Ethan".to_owned(),
        },

        SIVA {
            display_name = "Siva".to_owned(),
        },

        TERRA {
            display_name = "Terra".to_owned(),
        },
    }

    res_map
}
