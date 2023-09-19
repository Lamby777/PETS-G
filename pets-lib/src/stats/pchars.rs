//!
//! This module is like a registry of characters.
//!

/// "Playable Character"
/// This term might not make much sense, but it was used all over
/// the old code, and I'm already used to calling them that.
/// tl;dr get used to it.
pub struct PChar;

impl PChar {
    // ------------------------------------------------------------
    // the "chosen ones"... :)
    // ------------------------------------------------------------
    pub const PORKY: &str = "Porky";
    pub const ETHAN: &str = "Ethan";
    pub const TERRA: &str = "Terra";
    pub const SIVA: &str = "Siva";

    // ------------------------------------------------------------
    // These guys will definitely be playable in the main storyline.
    // ------------------------------------------------------------
    pub const DYLAN: &str = "Dylan";
    pub const MIRA: &str = "Mira";

    // ------------------------------------------------------------
    // not sure if these guys are gonna be playable in the main game...
    // either way, they still have side-stories so they still need PChars
    // ------------------------------------------------------------
    pub const FUZZY: &str = "Fuzzy";
    pub const LEO: &str = "Leo";
    pub const LYEMBO: &str = "L'yembo";

    // ------------------------------------------------------------
    // These guys MIGHT be playable at some point...
    // ------------------------------------------------------------
    pub const WINTHRUS: &str = "Winthrus";
    pub const JUNO: &str = "Juno";
    pub const HASSAN: &str = "Hassan";
    pub const NYX: &str = "Nyx";
    pub const QUOLO: &str = "Quolo";
    pub const BOBBY: &str = "Bobby";
}
