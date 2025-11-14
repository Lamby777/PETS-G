//!
//! This module is for processing placeholders in dialogue strings.
//!

use crate::common::*;
use godot::prelude::*;

// get party leader id as a `String` (helper fn to reduce boilerplate)
pub fn leader() -> String {
    si().bind().save.party[0].to_string()
}

pub fn fmt_possibly_custom_display_name(pchar: &str) -> String {
    // TODO: if pchar was given a custom name by the player, use it

    // else default to localized name
    format!("DG_SPK_{pchar}")
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
    ("[PORKY]", || fmt_possibly_custom_display_name("Porky")),
    ("[ETHAN]", || fmt_possibly_custom_display_name("Ethan")),
    ("[TERRA]", || fmt_possibly_custom_display_name("Terra")),
    ("[SIVA]", || fmt_possibly_custom_display_name("Siva")),
    ("[LYEMBO]", || fmt_possibly_custom_display_name("Lyembo")),
    ("[QUOLO]", || fmt_possibly_custom_display_name("Quolo")),
    ("[JUNIPER]", || {
        match leader().as_str() {
            "Ethan" => "DG_SPK_MOM",
            _ => "DG_SPK_JUNIPER",
        }
        .to_owned()
    }),
    ("[CLAY]", || {
        match leader().as_str() {
            "Ethan" => "DG_SPK_DAD",
            _ => "DG_SPK_CLAY",
        }
        .to_owned()
    }),
    ("[MR_TULIVAE]", || {
        match leader().as_str() {
            "Siva" => "DG_SPK_DAD",
            _ => "DG_SPK_MR_TULIVAE",
        }
        .to_owned()
    }),
    ("[MRS_TULIVAE]", || {
        match leader().as_str() {
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
