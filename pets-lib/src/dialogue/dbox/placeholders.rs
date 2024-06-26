//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::prelude::*;

pub fn party_leader() -> PChar {
    *PlayerCB::singleton().bind().party_pchars().first().unwrap()
}

const PLACEHOLDERS: &[(&'static str, fn() -> String)] = &[
    ("[PLAYER]", || "Cherry".to_string()),
    ("[LEVEL]", || 123.to_string()),
    // NOTE <https://github.com/Lamby777/PETS-G/issues/23>
    ("[ETHAN]", || "Ethan".to_string()),
    ("[MOM]", || {
        match party_leader() {
            PChar::ETHAN => "DG_SPK_MOM",
            _ => "DG_SPK_PAULA",
        }
        .to_string()
    }),
    ("[DAD]", || {
        match party_leader() {
            PChar::ETHAN => "DG_SPK_DAD",
            _ => "DG_SPK_CLAY",
        }
        .to_string()
    }),
    ("[MR_TULIVAE]", || {
        match party_leader() {
            PChar::SIVA => "DG_SPK_DAD",
            _ => "DG_SPK_MR_TULIVAE",
        }
        .to_string()
    }),
    ("[MRS_TULIVAE]", || {
        match party_leader() {
            PChar::SIVA => "DG_SPK_MOM",
            _ => "DG_SPK_MRS_TULIVAE",
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
