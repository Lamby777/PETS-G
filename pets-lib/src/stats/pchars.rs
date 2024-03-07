//!
//! This module is like a registry of characters.
//!

/// "Playable Character"
/// This term might not make much sense, but it was used all over
/// the old code, and I'm already used to calling them that.
/// tl;dr get used to it.
pub struct PChar;
pub struct EnemyID;

macro_rules! names {
    ($($name:ident),* $(,)?) => {
        $(
            #[allow(unused)]
            pub const $name: &'static str = stringify!($name);
        )*

        pub const ALL: &'static [&'static str] = &[$(stringify!($name)),*];
    };
}

impl PChar {
    names! {
        // Debugging purposes
        CHERRY,

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
        // Probably not, but either way they still need PChars
        // and some have side stories
        WINTHRUS,
        JUNO,
        COLT,
        HASSAN,
        NYX,
        QUOLO,
        BOBBY,
    }
}

#[allow(unused)]
impl EnemyID {
    names! {
        ANONNYMOUSE,
    }
}
