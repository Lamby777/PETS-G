//!
//! This module is for character/enemy stat type definitions
//!

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::items::Item;

pub mod pchars;
pub mod savefiles;
pub mod state;

pub type CharMap = HashMap<String, Rc<RefCell<CharData>>>;
pub type IntegralStat = i16;

/// All the information the game needs to know about a character
#[derive(Debug, Serialize, Deserialize)]
pub struct CharData {
    /// Name of the character
    name: String,

    // TODO following the "YAGNI" principle, I'm gonna stop adding
    // more complicated shit here. Later on, we should prob have
    // a way to make "base" stats affect the regular stat increase
    // levels in both linear and constant ways. Maybe a whole separate
    // impl for getting a stat, where the functions just do all
    // the math under the hood? Sounds like a lot of boilerplate...
    /// "Base" stats... Some characters are just better at some
    /// things than others, right?
    base_stats: CharStats,

    /// The character's long-term stats
    /// "Core" stats and maximums, pretty much
    stats: CharStats,

    /// The character's short-term stats
    /// Stuff like how full a bar is, etc.
    // seriously hope this won't cause a typo later
    state: CharStateful,

    /// Status conditions the character has
    conditions: HashSet<StatusConditions>,

    /// Items this character is holding
    inventory: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharStateful {
    /// Current HP
    hp: IntegralStat,

    /// Current energy level
    energy: IntegralStat,
    // mana starts at 0 each battle
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharStats {
    max_hp: IntegralStat,
    max_energy: IntegralStat,

    attack: IntegralStat,
    defense: IntegralStat,
    speed: IntegralStat,
    stability: IntegralStat,

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    delta: IntegralStat,
    epsilon: IntegralStat,

    // Exclusive to certain characters
    // NOTE maybe use traits for this?
    // idk the overhead of dynamic dispatch might not be worth it
    lambda: Option<IntegralStat>,
    max_mana: Option<IntegralStat>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
