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
pub use savefiles::SaveFile;
pub use statcalc::{CharStatCalcs, StatCalcFn, StatCalcList};

// type aliases
pub type CharMap = HashMap<String, RefCell<CharData>>;
pub type IntegralStat = i16;
pub type FloatStat = f32;

/// Trait for stuff that both party members and enemies
/// have. For example, an enemy doesn't need to have a
/// "level," but it does need to have HP and status effects.
trait Battler {
    fn hp_mut(&mut self) -> &mut IntegralStat;
    fn max_hp(&self) -> IntegralStat;
    fn status_effects(&self) -> &HashSet<StatusEffect>;
    fn status_effects_mut(&mut self) -> &mut HashSet<StatusEffect>;

    /// Subtract damage count from the character's HP
    ///
    /// Saturated at 0.
    fn take_damage(&mut self, damage: IntegralStat) {
        let hp = *self.hp_mut();
        *self.hp_mut() = 0.max(hp - damage);
    }

    /// Add back HP to the character
    ///
    /// Saturated at the character's max HP
    fn heal(&mut self, amount: IntegralStat) {
        let hp = *self.hp_mut();
        let max_hp = self.max_hp();
        *self.hp_mut() = max_hp.min(hp + amount);
    }

    fn apply_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects_mut().insert(effect);
    }

    fn remove_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects_mut().remove(&effect);
    }

    fn has_status_effect(&self, effect: StatusEffect) -> bool {
        self.status_effects().contains(&effect)
    }
}

/// All the information the game needs to know about a character
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharData {
    /// Name of the character, as picked by the user
    /// ⚠️⚠️⚠️ See <https://github.com/Lamby777/PETS-G/issues/23>
    pub display_name: String,

    pub level: IntegralStat,
    pub stats: CharStats,

    /// Status effects the character has
    pub status_effects: HashSet<StatusEffect>,

    /// Items this character is holding
    pub inventory: Vec<Item>,
}

impl Default for CharData {
    fn default() -> Self {
        CharData {
            display_name: "Chicken Nugget".to_owned(),
            level: 1,
            stats: Default::default(),
            status_effects: Default::default(),
            inventory: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CharStats {
    pub stateless: InherentStats,
    pub stateful: CharStatsStateful,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CharStatsStateful {
    /// Current HP
    pub hp: IntegralStat,

    /// Current energy level
    pub energy: IntegralStat,
    // mana starts at 0 each battle
}

/// Stats that are inherent to a character.
///
/// This doesn't mean they never change; they do every
/// time you level up or use certain items...
///
/// It just means "inherent" as in it doesn't constantly
/// change like HP, mana, or energy.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InherentStats {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
