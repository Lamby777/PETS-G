//!
//! This module is for processing placeholders in dialogue strings.
//!

const PLACEHOLDERS: &[(&'static str, fn() -> String)] = &[
    ("[PLAYER]", || "Cherry".to_string()),
    ("[LEVEL]", || 123.to_string()),
    // NOTE <https://github.com/Lamby777/PETS-G/issues/23>
    ("[ETHAN]", || "Ethan".to_string()),
    ("[MOM]", || "Paula".to_string()),
    ("[DAD]", || "Clay".to_string()),
];

pub fn process_placeholders(text: &str) -> String {
    let mut out = text.to_string();

    for (keyword, mapper) in PLACEHOLDERS {
        out = out.replace(keyword, &mapper());
    }

    out
}
