//!
//! This module is for character/enemy stat type definitions
//!

use crate::prelude::*;

// stat-related submodules
pub mod autoload;
pub mod charmap;
pub mod pchars;
pub mod savefiles;
pub mod statcalc;

// re-export some crap from ^^^
pub use autoload::StatsInterface;
pub use pchars::PChar;
pub use savefiles::SaveFile;
pub use statcalc::{CharStatCalcs, StatCalcFn, StatCalcList};

// type aliases
pub type CharMap = HashMap<String, RefCell<CharData>>;
pub type IntegralStat = i16;
pub type FloatStat = f32;

/// All the information the game needs to know about a character
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharData {
    /// Name of the character, as picked by the user
    /// ⚠️⚠️⚠️ See <https://github.com/Lamby777/PETS-G/issues/23>
    pub display_name: String,

    /// Level of the character
    pub level: IntegralStat,

    /// The character's long-term stat offsets
    /// Stuff like using a consumable with permanent boosts...
    pub stats: CharStats,

    /// The character's short-term stats
    /// Stuff like how full a bar is, etc.
    pub state: CharStatsStateful,

    /// Status conditions the character has
    pub conditions: HashSet<StatusEffect>,

    /// Items this character is holding
    pub inventory: Vec<Item>,
}

impl Default for CharData {
    fn default() -> Self {
        // will be dropped after this function...
        // just need it to see default values and prevent
        // repeating the same numbers everywhere
        let calc = StatCalcList::default();
        let level = 1;

        let state = CharStatsStateful {
            hp: (calc.max_hp)(level),
            energy: (calc.max_energy)(level),
        };

        CharData {
            display_name: "Chicken Nugget".to_owned(),
            level,
            state,

            stats: Default::default(),
            conditions: Default::default(),
            inventory: Default::default(),
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CharStats {
    pub max_hp: IntegralStat,
    pub max_energy: IntegralStat,

    pub attack: IntegralStat,
    pub defense: IntegralStat,
    pub speed: IntegralStat,
    pub stability: IntegralStat, // PK defense

    // refer to google doc for what these do...
    // can't pick a good name for em yet
    pub delta: IntegralStat,   // "the crit one"
    pub epsilon: IntegralStat, // "the combo one"

    // Exclusive to certain characters
    // NOTE maybe use traits for this?
    // idk the overhead of dynamic dispatch might not be worth it
    pub lambda: Option<IntegralStat>,
    pub max_mana: Option<IntegralStat>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum StatusEffect {
    Sleeping,    // Can't move, but recover 20% energy on wakeup
    Paralysis,   // ^^^ No movement, no energy recovery, but still has PK. Almost no combos
    Crying,      // Oops, all your attacks missed! Sowwy :<
    LightHeaded, // Like uncontrollable crying + also affects PK, but lower miss rate overall

    ShortBreath, // No attacks, painfully slow movement
    Dizziness,   // "Disoriented", auditory flashbang + harder combos
    Blinded,     // Battle board turns black

    Burn,      // Damage over time
    Frostbite, // More damage, introduced later in the game
    Bleeding,  // ^^^ HP meter biased towards rolling down faster
    Poison,    // ^^^ no PK
    PoisonR,   // ^^^ no PK, completely unable to fight (basically dead)

    Tired, // Less lenient music timing. Get some rest, dumbass! Don't emulate my bad habits.
}

impl Display for StatusEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StatusEffect::*;

        let s = match self {
            Sleeping => "Sleeping",
            Paralysis => "Paralyzed",
            Crying => "Crying",
            LightHeaded => "Light-headed",
            ShortBreath => "Short of breath",
            Dizziness => "Dizzy",
            Blinded => "Blinded",
            Burn => "Burn",
            Frostbite => "Frostbite",
            Bleeding => "Bleeding",
            Poison => "Poisoned",
            PoisonR => "Poisoned (R)",
            Tired => "Tired",
        };

        write!(f, "{}", s)
    }
}
