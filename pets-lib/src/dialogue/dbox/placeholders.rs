//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::prelude::*;

pub fn is_party_leader(pchar: &PChar) -> bool {
    let party = PlayerCB::singleton().bind().party_pchars();
    let leader = party.first().unwrap();

    dbg!(leader, pchar);
    leader == pchar
}

const PLACEHOLDERS: &[(&'static str, fn() -> String)] = &[
    ("[PLAYER]", || "Cherry".to_string()),
    ("[LEVEL]", || 123.to_string()),
    // NOTE <https://github.com/Lamby777/PETS-G/issues/23>
    ("[ETHAN]", || "Ethan".to_string()),
    ("[MOM]", || {
        match is_party_leader(&PChar::ETHAN) {
            true => "Mom",
            false => "Paula",
        }
        .to_string()
    }),
    ("[DAD]", || {
        match is_party_leader(&PChar::ETHAN) {
            true => "Dad",
            false => "Clay",
        }
        .to_string()
    }),
];

pub fn process_placeholders(text: &str) -> String {
    let mut out = text.to_string();

    for (keyword, mapper) in PLACEHOLDERS {
        out = out.replace(keyword, &mapper());
    }

    out
}
