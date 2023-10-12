//!
//! This module is like a registry of characters.
//!

/// "Playable Character"
/// This term might not make much sense, but it was used all over
/// the old code, and I'm already used to calling them that.
/// tl;dr get used to it.
pub struct PChar;

macro_rules! pchar_names {
    ($($name:ident),*) => {
        $(pub const $name: &'static str = stringify!($name);)*

        pub const ALL: &'static [&'static str] = &[$(stringify!($name)),*];
    };
}

impl PChar {
    pchar_names! {
        // the "chosen ones"... :)
        PORKY,
        ETHAN,
        TERRA,
        SIVA,

        // These guys will def be playable in the main storyline.
        DYLAN,
        MIRA,

        // not sure if these guys will be playable in the main game,
        // but they still have side-stories so they'd still need PChars
        FUZZY,
        LEO,
        LYEMBO,

        // These guys MIGHT be playable at some point...
        WINTHRUS,
        JUNO,
        COLT,
        HASSAN,
        NYX,
        QUOLO,
        BOBBY
    }
}
