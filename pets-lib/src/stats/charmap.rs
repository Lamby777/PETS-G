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

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> CharMap {
    let mut res = uniform_charmap();

    // Ethan's special stuff
    res.entry(PChar::ETHAN.to_owned()).and_modify(|pchar| {
        let mut pchar = pchar.borrow_mut();
        pchar.base_stats.lambda = Some(1);
        pchar.base_stats.max_mana = Some(20);
    });

    // Siva's special stuff
    res.entry(PChar::SIVA.to_owned()).and_modify(|pchar| {
        let mut pchar = pchar.borrow_mut();
        pchar.base_stats.max_mana = Some(20);
    });

    res
}
