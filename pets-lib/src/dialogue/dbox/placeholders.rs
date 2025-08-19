//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::common::*;
use godot::prelude::*;

pub fn party_leader_id() -> String {
    pcb().bind().party_pchars().first().unwrap().to_string()
}

pub fn pchar_display_name(pchar: &str) -> String {
    let sn = StringName::from(pchar);
    REGISTRIES.chars.get(&sn).unwrap().display_tr_key.clone()
}

// TODO: this shouldn't be hard-coded. use the Fn trait and
// allow registering new placeholders. Not a big concern rn tho
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
    ("[ETHAN]", || pchar_display_name("Ethan")),
    ("[LYEMBO]", || pchar_display_name("Lyembo")),
    ("[QUOLO]", || pchar_display_name("Quolo")),
    ("[JUNIPER]", || {
        match party_leader_id().as_str() {
            "Ethan" => "DG_SPK_MOM",
            _ => "DG_SPK_JUNIPER",
        }
        .to_owned()
    }),
    ("[CLAY]", || {
        match party_leader_id().as_str() {
            "Ethan" => "DG_SPK_DAD",
            _ => "DG_SPK_CLAY",
        }
        .to_owned()
    }),
    ("[MR_TULIVAE]", || {
        match party_leader_id().as_str() {
            "Siva" => "DG_SPK_DAD",
            _ => "DG_SPK_MR_TULIVAE",
        }
        .to_owned()
    }),
    ("[MRS_TULIVAE]", || {
        match party_leader_id().as_str() {
            "Siva" => "DG_SPK_MOM",
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
