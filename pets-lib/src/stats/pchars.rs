//!
//! This module is like a registry of characters.
//!
//! "Playable Character"
//! This term might not make much sense, but it was used all over
//! the old code, and I'm already used to calling them that.
//! tl;dr get used to it.

use crate::prelude::*;

use godot::prelude::*;
use strum::EnumIter;

//         //////////////////////////////////////////////////////////
//       ////                                          ////////////
//     //////   POTENTIAL SPOILERS BEYOND THIS POINT   //////////
//   ////////                                          ////////
// //////////////////////////////////////////////////////////

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize, EnumIter)]
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum PChar {
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
    NEOXYLIN,
    QUOLO,

    // Silly billies
    RUMBLE,
    ESTHER,
    RANA,
    JERIAN,

    // Families
    JUNIPER,
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
    RAINE, // GLUE arc
    TAINE, // GLUE arc

    // Shopkeepers
    CHERRY,      // Cherry's Cherries
    RICHARD,     // Big Richard's Big Ripoffs
    CHAIRPERSON, // Chair Chairperson of the Gaming Chair Committee
}

impl Display for PChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(unused, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize, EnumIter)]
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum EnemyID {
    A_NONNY_MOUSE,
    COPPER_CROW,
    XLR8,
}

impl Display for EnemyID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
