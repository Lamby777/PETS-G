//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::common::*;

pub fn party_leader() -> PChar {
    *pcb().bind().party_pchars().first().unwrap()
}

pub fn pchar_display_name(pchar: &PChar) -> String {
    si().bind()
        .get_character(pchar)
        .borrow()
        .display_name
        .clone()
}

type PlaceholderMapping = (&'static str, fn() -> String);
const PLACEHOLDERS: &[PlaceholderMapping] = &[
    ("[PLAYER]", || "Cherry".to_owned()),
    ("[LEVEL]", || 123.to_string()),
    // "special" speakers
    ("[NARRATOR]", || "".to_owned()),
    ("[???]", || "DG_SPK_UNKNOWN".to_owned()),
    // character names
    ("[CASCADE]", || "DG_SPK_CASCADE".to_owned()),
    ("[RODRICK]", || "DG_SPK_RODRICK".to_owned()),
    ("[ETHAN]", || pchar_display_name(&PChar::Ethan)),
    ("[LYEMBO]", || pchar_display_name(&PChar::Lyembo)),
    ("[QUOLO]", || pchar_display_name(&PChar::Quolo)),
    ("[JUNIPER]", || {
        match party_leader() {
            PChar::Ethan => "DG_SPK_MOM",
            _ => "DG_SPK_JUNIPER",
        }
        .to_owned()
    }),
    ("[CLAY]", || {
        match party_leader() {
            PChar::Ethan => "DG_SPK_DAD",
            _ => "DG_SPK_CLAY",
        }
        .to_owned()
    }),
    ("[MR_TULIVAE]", || {
        match party_leader() {
            PChar::Siva => "DG_SPK_DAD",
            _ => "DG_SPK_MR_TULIVAE",
        }
        .to_owned()
    }),
    ("[MRS_TULIVAE]", || {
        match party_leader() {
            PChar::Siva => "DG_SPK_MOM",
            _ => "DG_SPK_MRS_TULIVAE",
        }
        .to_owned()
    }),
];

pub fn process_placeholders(text: &str) -> String {
    let mut out = text.to_owned();

    for (keyword, mapper) in PLACEHOLDERS {
        out = out.replace(keyword, &mapper());
    }

    out
}
