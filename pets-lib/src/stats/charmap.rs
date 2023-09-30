use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    let mut res = CharMap::new();
    let deft_stats = CharData::default();

    // add chars from registry
    for chname in PChar::ALL.iter() {
        let cloned = deft_stats.clone();
        res.insert(chname.to_string(), Rc::new(RefCell::new(cloned)));
    }

    res
}

/// "Jat Chippity goes hard"
/// Makes it easier to write custom base stats and stuff
macro_rules! ch_unique {
    ($map:expr, $character:expr, $($field:ident $(.$property:ident)? = $value:expr),*) => {
        $map.entry($character.to_owned()).and_modify(|pchar| {
            let mut pchar = pchar.borrow_mut();
            $(pchar.$field$(.$property)? = $value;)*
        });
    };
}

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> CharMap {
    let mut res = uniform_charmap();

    // Ethan's special stuff
    ch_unique!(
        res,
        PChar::ETHAN,
        name = "Ethan".to_string(),
        base_stats.lambda = Some(1),
        base_stats.max_mana = Some(1)
    );

    // Siva's special stuff
    ch_unique!(
        res,
        PChar::SIVA,
        name = "Siva".to_string(),
        base_stats.max_mana = Some(1)
    );

    res
}
