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

    /// Items this character is holding
    inventory: Vec<items::Item>,
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

    attack: u16,
    defense: u16,
    speed: u16,
    stability: u16,

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    delta: u16,
    epsilon: u16,
    lambda: u16, // might keep this one's name tho, baa :)
}

pub enum StatusConditions {
    Sleeping,    // Can't move, but recover 20% energy on wakeup
    Paralysis,   // ^^^ No movement, no energy recovery, but still has PK. Almost no combos
    Crying,      // Oops, all your attacks missed! Sowwy :<
    LightHeaded, // Like uncontrollable crying + also affects PK, but lower miss rate overall

    ShortBreath, // No attacks, painfully slow movement
    Dizziness,   // "Disoriented", auditory flashbang + harder combos
    Blinded,     // Battle board turns black

    Heatstroke, // Damage over time
    Frostbite,  // More damage, introduced later in the game
    Bleeding,   // ^^^ HP meter biased towards rolling down faster
    Poison,     // ^^^ no PK
    PoisonR,    // ^^^ no PK, completely unable to fight (basically dead)

    Tired, // Less lenient music timing. Get some rest, dumbass! Don't emulate my bad habits.
}
