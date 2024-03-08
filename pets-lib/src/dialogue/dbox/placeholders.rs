//!
//! This module is for processing placeholders in dialogue strings.
//! (duh)
//!

// use super::*;

type DPlaceholder = (&'static str, fn() -> String);
const PLACEHOLDERS: &[DPlaceholder] = &[
    ("[PLAYER]", || "Cherry".to_string()),
    ("[LEVEL]", || 123.to_string()),
];

pub fn process_placeholders(text: &str) -> String {
    let mut out = text.to_string();

    for (keyword, mapper) in PLACEHOLDERS {
        out = out.replace(keyword, &mapper());
    }

    out
}
