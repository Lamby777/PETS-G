//!
//! This module is for character/enemy stat type definitions
//!

use crate::common::*;
use godot::prelude::*;

// stat-related submodules
mod autoload;
mod battler;
mod charmap;
mod enemy;
mod pchars;
mod quests;
mod savefiles;
pub mod scrapbook;
mod statcalc;

// re-export some crap from ^^^
pub use autoload::StatsInterface;
pub use battler::{Battler, Battlers};
pub use charmap::CharMap;
pub use enemy::EnemyData;
pub use pchars::{EnemyID, PChar};
pub use quests::QuestPhase;
pub use savefiles::SaveFile;

/// Stats that aren't inherent and change via battles or
/// occasionally other things. "Current" stuff like HP, Mana, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleStats {
    pub hp: IntegralStat,
    pub mana: Option<IntegralStat>,
    pub energy: IntegralStat,
    pub buffs: Vec<InherentStats>,
}

impl Default for BattleStats {
    fn default() -> Self {
        BattleStats {
            hp: 30,
            mana: None,
            energy: 1,
            buffs: vec![],
        }
    }
}

/// All the information the game needs to know about a character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharData {
    pub id: PChar,

    /// Name of the character, as picked by the user
    /// ⚠️⚠️⚠️ See <https://github.com/Lamby777/PETS-G/issues/23>
    pub display_name: String,
    pub level: IntegralStat,

    pub battler: Rc<RefCell<Battler>>,
    pub weapon_style: DefaultWeaponStyle,
}

/// Stats that are inherent to a character or enemy type.
///
/// This doesn't mean they never change; they do every
/// time you level up or use certain items...
///
/// It just means "inherent" as in it doesn't constantly
/// change like HP, mana, or energy.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    derive_more::Sum,
    Serialize,
    Deserialize
)]
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
    // Spoiler alert: Ethan is not the only one who needs lambda... :>
    // EDIT: I forgot what the above line is referring to LMFAO
    pub lambda: Option<IntegralStat>,
    pub max_mana: Option<IntegralStat>,
}

impl Default for InherentStats {
    fn default() -> Self {
        InherentStats {
            max_hp: 30,
            max_energy: 1,
            attack: 1,
            defense: 1,
            speed: 1,
            stability: 1,
            delta: 1,
            epsilon: 1,
            lambda: None,
            max_mana: None,
        }
    }
}

impl InherentStats {
    pub fn zero() -> Self {
        InherentStats {
            max_hp: 0,
            max_energy: 0,
            attack: 0,
            defense: 0,
            speed: 0,
            stability: 0,
            delta: 0,
            epsilon: 0,
            lambda: None,
            max_mana: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffect {
    /// Can't move, but recover 20% energy on wakeup
    Sleeping,

    /// ^^^ No movement, no energy recovery, but still has PK. Almost no combos
    Paralyzed,

    /// Oops, all your attacks missed! Sowwy :<
    Crying,

    /// Like uncontrollable crying + also no PK, but lower miss rate overall
    LightHeaded,

    /// No attacks, painfully slow movement
    ShortBreath,

    /// "Disoriented", auditory flashbang + harder combos
    Dizzy,

    /// Battle board turns black
    Blinded,

    /// Damage over time
    Burning,

    /// More damage over time, introduced later in the game
    Frostbite,

    /// ^^^ HP meter biased towards rolling down faster
    Bleeding,

    /// ^^^ no PK
    Poison,

    /// ^^^ no PK, completely unable to fight (basically dead)
    PoisonR,

    /// Less lenient music timing.
    Tired,

    /// No physical attacks, but PK is fine
    Disarmed,
}

impl StatusEffect {
    pub fn rating(&self) -> u8 {
        use StatusEffect::*;
        match self {
            Paralyzed => 1,
            Crying => 1,
            Sleeping => 1,

            LightHeaded => 2,
            Bleeding => 2,
            Poison => 2,

            Blinded => 3,
            Burning => 3,
            Frostbite => 3,

            ShortBreath => 4,
            Dizzy => 4,
            Tired => 4,

            PoisonR => 5,

            // cannot be healed
            Disarmed => 6,
        }
    }
}

impl Display for StatusEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StatusEffect::*;

        let s = match self {
            LightHeaded => "Light-headed",
            ShortBreath => "Short of breath",
            Burning => "On Fire",
            PoisonR => "Poisoned (R)",
            _ => return Debug::fmt(&self, f),
        };

        write!(f, "{}", s)
    }
}

fn add_options<T>(a: Option<T>, b: Option<T>) -> Option<T>
where
    T: std::ops::Add<Output = T>,
{
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn sub_options<T>(a: Option<T>, b: Option<T>) -> Option<T>
where
    T: std::ops::Sub<Output = T>,
{
    match (a, b) {
        (Some(a), Some(b)) => Some(a - b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Types of weapons that a character can use in their default "rhythm" attacks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum DefaultWeaponStyle {
    /// no weapon has this type, therefore a character who can only equip
    /// this enum variant can't equip anything.
    CannotEquip,

    Blade,
    Gun,
    Debug,
}

// WARNING: The stuff below is mostly just boilerplate garbage generated by copilot.

impl Default for CharData {
    fn default() -> Self {
        CharData {
            id: PChar::Devon,
            display_name: "Chicken Nugget".to_owned(),
            level: 1,

            // i seriously can't `..Default::default()` because
            // that would be infinite recursion... WTF?
            battler: Default::default(),
            weapon_style: DefaultWeaponStyle::Blade,
        }
    }
}

impl std::ops::Add for InherentStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        InherentStats {
            max_hp: self.max_hp + rhs.max_hp,
            max_energy: self.max_energy + rhs.max_energy,
            attack: self.attack + rhs.attack,
            defense: self.defense + rhs.defense,
            speed: self.speed + rhs.speed,
            stability: self.stability + rhs.stability,
            delta: self.delta + rhs.delta,
            epsilon: self.epsilon + rhs.epsilon,

            lambda: add_options(self.lambda, rhs.lambda),
            max_mana: add_options(self.max_mana, rhs.max_mana),
        }
    }
}

impl std::ops::Sub for InherentStats {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        InherentStats {
            max_hp: self.max_hp - rhs.max_hp,
            max_energy: self.max_energy - rhs.max_energy,
            attack: self.attack - rhs.attack,
            defense: self.defense - rhs.defense,
            speed: self.speed - rhs.speed,
            stability: self.stability - rhs.stability,
            delta: self.delta - rhs.delta,
            epsilon: self.epsilon - rhs.epsilon,

            lambda: sub_options(self.lambda, rhs.lambda),
            max_mana: sub_options(self.max_mana, rhs.max_mana),
        }
    }
}
