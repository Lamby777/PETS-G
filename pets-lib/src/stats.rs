//!
//! This module is for character/enemy stat type definitions
//!

use std::collections::HashSet;

pub struct CharData {
    /// Name of the character
    name: String,

    /// The character's long-term stats
    /// "Base" stats and maximums, pretty much
    stats: CharStats,

    /// The character's short-term stats
    /// Stuff like how full a bar is, etc.
    // seriously hope this won't cause a typo later
    state: CharStateful,

    /// Status conditions the character has
    conditions: HashSet<StatusConditions>,
}

pub struct CharStateful {
    /// Current HP
    hp: u16,

    /// Current energy level
    energy: u16,
    // mana starts at 0 each battle
}

pub struct CharStats {
    max_hp: u16,
    max_mana: u16,
    max_energy: u16,
    speed: u16,
    stability: u16,

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    delta: u16,
    epsilon: u16,
    lambda: u16, // might keep this one's name tho, baa :)
}

pub enum StatusConditions {
    Sleeping,
    Paralysis,
    Crying,

    ShortBreath,
    Dizziness,
    Blinded,

    // maybe refactor this part (bruh moment)
    Heatstroke,
    Frostbite,
    Bleeding,
    Poison,
    PoisonR,

    Tired,
    LightHeaded,
}
