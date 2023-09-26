//!
//! This module is for character/enemy stat type definitions
//!

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub mod pchars;
pub mod savefiles;
pub mod state;

pub type CharMap = HashMap<String, Rc<RefCell<CharData>>>;
pub type IntegralStat = i16;
pub type FloatStat = f32;

/// All the information the game needs to know about a character
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharData {
    /// Name of the character
    pub name: String,

    // TODO following the "YAGNI" principle, I'm gonna stop adding
    // more complicated shit here. Later on, we should prob have
    // a way to make "base" stats affect the regular stat increase
    // levels in both linear and constant ways. Maybe a whole separate
    // impl for getting a stat, where the functions just do all
    // the math under the hood? Sounds like a lot of boilerplate...
    /// "Base" stats... Some characters are just better at some
    /// things than others, right?
    pub base_stats: CharStats,

    /// The character's long-term stats
    /// "Core" stats and maximums, pretty much
    pub stats: CharStats,

    /// The character's short-term stats
    /// Stuff like how full a bar is, etc.
    pub state: CharStatsStateful,

    /// Status conditions the character has
    pub conditions: HashSet<StatusConditions>,

    /// Items this character is holding
    pub inventory: Vec<Item>,
}

impl Default for CharData {
    fn default() -> Self {
        // This part is a bit ugly...

        let base_stats = CharStats {
            max_hp: 20,
            max_energy: 10,

            attack: 1,
            defense: 0,
            speed: 1,
            stability: 40,
            delta: 0,
            epsilon: 1,

            max_mana: None,
            lambda: None,
        };

        let stats = CharStats {
            max_hp: 0,
            max_energy: 0,

            attack: 0,
            defense: 0,
            speed: 0,
            stability: 0,
            delta: 0,
            epsilon: 0,

            max_mana: Some(0),
            lambda: Some(0),
        };

        let state = CharStatsStateful {
            hp: base_stats.max_hp,
            energy: base_stats.max_energy,
        };

        CharData {
            name: "Chicken Nugget".to_owned(),
            base_stats,
            stats,
            state,
            conditions: HashSet::new(),
            inventory: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharStatsStateful {
    /// Current HP
    pub hp: IntegralStat,

    /// Current energy level
    pub energy: IntegralStat,
    // mana starts at 0 each battle
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharStats {
    pub max_hp: IntegralStat,
    pub max_energy: IntegralStat,

    pub attack: IntegralStat,
    pub defense: IntegralStat,
    pub speed: IntegralStat,
    pub stability: IntegralStat,

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    pub delta: IntegralStat,
    pub epsilon: IntegralStat,

    // Exclusive to certain characters
    // NOTE maybe use traits for this?
    // idk the overhead of dynamic dispatch might not be worth it
    pub lambda: Option<IntegralStat>,
    pub max_mana: Option<IntegralStat>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
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
