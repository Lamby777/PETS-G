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
    let mut map = uniform_charmap();

    ch_unique_registry! {
        map,

        ETHAN {
            display_name = "Ethan".to_owned(),

            inherent_stats_base.max_hp = -10,
            inherent_stats_base.max_mana = Some(5),
            inherent_stats_base.attack = 1,
            inherent_stats_base.defense = 2,
            inherent_stats_base.speed = 4,
            inherent_stats_base.stability = 5,
            inherent_stats_base.delta = 5,
            inherent_stats_base.epsilon = 1,
            inherent_stats_base.lambda = Some(1),
        },

        TERRA {
            display_name = "Terra".to_owned(),
            level = 5,

            inherent_stats_base.max_hp = 35,
            inherent_stats_base.attack = 5,
            inherent_stats_base.defense = 4,
            inherent_stats_base.speed = 1,
            inherent_stats_base.stability = 3,
            inherent_stats_base.delta = 2,
            inherent_stats_base.epsilon = 1,
        },

        SIVA {
            display_name = "Siva".to_owned(),

            inherent_stats_base.max_mana = Some(10),
            inherent_stats_base.attack = 3,
            inherent_stats_base.defense = 1,
            inherent_stats_base.speed = 2,
            inherent_stats_base.stability = 5,
            inherent_stats_base.delta = 3,
            inherent_stats_base.epsilon = 1,
        },
    }

    // set everyone's hp to their max
    for (_, chardata) in map.iter_mut() {
        let mut pchar = chardata.borrow_mut();
        pchar.battle_stats.hp = pchar.inherent_stats().max_hp;
    }

    map
}
