//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::prelude::*;

pub fn party_leader() -> PChar {
    *pcb().bind().party_pchars().first().unwrap()
}

pub fn pchar_display_name(pchar: &PChar) -> String {
    si().bind().get_character(pchar).display_name
}

const PLACEHOLDERS: &[(&'static str, fn() -> String)] = &[
    ("[PLAYER]", || "Cherry".to_string()),
    ("[LEVEL]", || 123.to_string()),
    ("[ETHAN]", || pchar_display_name(&PChar::ETHAN)),
    ("[LYEMBO]", || pchar_display_name(&PChar::LYEMBO)),
    ("[QUOLO]", || pchar_display_name(&PChar::QUOLO)),
    ("[JUNIPER]", || {
        match party_leader() {
            PChar::ETHAN => "DG_SPK_MOM",
            _ => "DG_SPK_JUNIPER",
        }
        .to_string()
    }),
    ("[CLAY]", || {
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
