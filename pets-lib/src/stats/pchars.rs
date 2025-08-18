//!
//! This module is like a registry of characters.
//!
//! # "Playable Character"
//!
//! This term might not make much sense anymore, because not all PChars
//! represent playable characters, but it was used all over the old code,
//! and I'm already used to calling them that.
//!
//! tl;dr get used to it.

use crate::common::*;

use godot::prelude::*;
use strum::EnumIter;

//         //////////////////////////////////////////////////////////
//       ////                                          ////////////
//     //////   POTENTIAL SPOILERS BEYOND THIS POINT   //////////
//   ////////                                          ////////
// //////////////////////////////////////////////////////////

/// ID of any playable (or in rare cases non-playable) character
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize, EnumIter)]
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum PChar {
    // the "chosen ones"... :)
    Porky,
    Ethan,
    Terra,
    Siva,

    // These guys will def be playable in the main storyline.
    Dylan,
    Mira,
    Leo,

    // The rest probably won't be playable in the main game,
    // but need PChars regardless.
    Devon,
    Fuzzy,
    Lyembo,
    Quolo,
    // Bobby,
    // Neoxylin,

    // Silly billies
    // Rumble,
    // Esther,
    // Rana,
    // Jerian,

    // Families
    Juniper,
    Clay,
    MrTulivae,
    MsTulivae,
    // Jolainne,

    // Winthrus, his crew, and associates
    // Winthrus,
    // Juno,
    // Colt,

    // Important Goober people
    // Mylo,  // HQ
    // Calyx, // Ren
    // Raine, // GLUE arc
    // Taine, // GLUE arc

    // Shopkeepers
    // Cherie,      // Cherie's Cherries
    // Richard,     // Big Richard's Big Ripoffs
    // Chairperson, // Chair Chairperson of the Gaming Chair Committee
}

impl Display for PChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize, EnumIter)]
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum EnemyID {
    ANonnyMouse,
    CopperCrow,
    XLR8,
}

impl Display for EnemyID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
