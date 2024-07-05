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

    ch_unique_registry! {
        res_map,

        ETHAN {
            display_name = "Ethan".to_owned(),

            inherent_stats.max_hp = 20,
            inherent_stats.max_mana = Some(10),
            inherent_stats.max_energy = 1,

            inherent_stats.attack = 1,
            inherent_stats.defense = 2,
            inherent_stats.speed = 4,
            inherent_stats.stability = 5,
            inherent_stats.delta = 5,
            inherent_stats.epsilon = 1,
            inherent_stats.lambda = Some(1),
        },

        SIVA {
            display_name = "Siva".to_owned(),
            inherent_stats.max_mana = Some(10),
        },

        TERRA {
            display_name = "Terra".to_owned(),
        },
    }

    res_map
}
