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

//         //////////////////////////////////////////////////////////
//       ////                                          ////////////
//     //////   POTENTIAL SPOILERS BEYOND THIS POINT   //////////
//   ////////                                          ////////
// //////////////////////////////////////////////////////////

impl PChar {
    names! {
        // the "chosen ones"... :)
        PORKY,
        ETHAN,
        TERRA,
        SIVA,

        // These guys will def be playable in the main storyline.
        DYLAN,
        MIRA,
        LEO,

        // The rest probably won't be playable in the main game,
        // but need PChars regardless.
        DEVON,
        FUZZY,
        LYEMBO,
        BOBBY,
        NYX,
        QUOLO,

        // Silly billies
        RUMBLE,
        ESTHER,
        RANA,
        JERIAN,

        // Families
        PAULA,
        CLAY,
        MR_TULIVAE,
        MS_TULIVAE,
        JOLAINNE,

        // Winthrus, his crew, and associates
        WINTHRUS,
        JUNO,
        HASSAN,
        COLT,

        // Important Goober people
        MYLO,  // HQ
        CALYX, // Ren
        RAINE, // VUE arc
        TAINE, // VUE arc

        // Shopkeepers
        CHERRY,      // Cherry's Cherries
        RICHARD,     // Big Richard's Big Ripoffs
        CHAIRPERSON, // Chair Chairperson of the Gaming Chair Committee
    }
}

#[allow(unused)]
impl EnemyID {
    names! {
        A_NONNY_MOUSE,
        COPPER_CROW,
        XLR8,
    }
}
