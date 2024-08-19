use crate::common::*;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Serialize, Deserialize)]
pub struct Scrapbook {
    // just IDs, will match up with the actual content and
    // count as unlocked if the ID is in here
    pub dev_notes_unlocked: HashSet<String>,
    pub pen_art_unlocked: HashSet<String>,
    pub ventures_unlocked: HashSet<String>,

    /// The player's progress in the battle memory.
    pub battle_memory: BattleMemory,
}

impl Scrapbook {
    pub fn empty() -> Self {
        Self {
            dev_notes_unlocked: HashSet::new(),
            pen_art_unlocked: HashSet::new(),
            ventures_unlocked: HashSet::new(),
            battle_memory: BattleMemory {
                entries: HashMap::new(),
                tracks: HashSet::new(),
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BattleMemory {
    /// KV pairs of the enemy's ID and what the player knows about it.
    pub entries: HashMap<String, BattleMemoryEntry>,

    /// Every track that the player has heard the beat to in battle.
    ///
    /// Having these unlocked allows you to turn on the beat at any
    /// time during a memory battle to practice timing your attacks.
    pub tracks: HashSet<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BattleMemoryEntry {
    sprite_front: bool,
    sprite_back: bool,

    /// Whether or not Porky has used Sniff on this enemy.
    /// If true, the player can see the enemy's affinities.
    sniffed: bool,
}
