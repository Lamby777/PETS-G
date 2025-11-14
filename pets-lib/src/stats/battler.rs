use super::*;

/// Any state related to health, equipment, battling, skills, etc. that
/// should be known about a character or enemy.
///
/// This is neutral ground. Do not write player-centered code here, as
/// this struct will be used exactly the same way for enemies as well.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Battler {
    /// All PChars have a level. This should ONLY EVER BE [None] FOR ENEMIES.
    ///
    /// Enemies don't need a level, because it's easier to tweak
    /// stats that way if I don't have to worry about offsets.
    pub level: Option<IntegralStat>,

    /// current hp/mana/energy, in-battle buffs, status fx, etc.
    ///
    /// this stuff is still useful outside of battles, but the point is
    /// it's not "inherent" to the character. when you go somewhere and
    /// heal up, all this stuff should be maxed out or cleared. it's NOT
    /// "improved" when you level up (besides like hp being bumped up a
    /// few points to match the increased max_hp).
    pub battle_stats: BattleStats,

    /// this is updated every time you get a permanent stat buff or something
    pub perm_buffs: LeveledStats,

    /// The IDs of all of the battler's equipped items
    pub equipment: Equipment,
}

impl Battler {
    /// Helper function to get the stats for a certain level.
    ///
    /// Returns incapable stats, so make sure you add it with capable ones
    /// or somehow process it if you need capable stats.
    fn leveled_stats_raw(&self) -> LeveledStats {
        match self.level {
            Some(lvl) => LeveledStats::from_level(lvl),
            None => LeveledStats::zero_all_incapable(),
        }
    }

    /// Subtract damage count from the character's HP, stopping at 0.
    /// Returns the new HP.
    pub fn take_damage(&mut self, damage: IntegralStat) -> IntegralStat {
        let hp = &mut self.battle_stats.hp;
        *hp = 0.max(*hp - damage);
        *hp
    }

    /// Add back HP to the character
    ///
    /// Saturated at the character's max HP
    pub fn heal(&mut self, amount: IntegralStat) {
        let max_hp = self.practical_stats().max_hp;
        let hp = &mut self.battle_stats.hp;
        *hp = max_hp.min(*hp + amount);
    }

    fn recover_mana(&mut self, amount: IntegralStat) {
        let Some(max_mana) = self.practical_stats().max_mana else {
            return; // nothing to do
        };

        // WARN: this will silently continue, recovering up from 0 if the
        // character has a `max_mana` but their `mana` is somehow `None`.
        let mana = &mut self.battle_stats.mana.unwrap_or(0);
        *mana = max_mana.min(*mana + amount);
    }

    /// This should take armor, weapons, etc. into account for players.
    /// It should NOT consider in-battle buffs/debuffs.
    fn armored_stats(&self) -> LeveledStats {
        self.leveled_stats_raw() + self.equipment.offsets()
    }

    /// The final "in practice" stats of the character. No more processing.
    /// For almost all intents and purposes, these are the actual stats that
    /// should be used for applying damage and stuff.
    ///
    /// Takes into account the...
    /// * Stats from leveling up
    /// * Equipment
    /// * Permanent buffs
    /// * Temporary battle-related buffs
    pub fn practical_stats(&self) -> LeveledStats {
        let armored = self.armored_stats();

        // don't forget to thank the buffs driver
        let battle_buffs = self.battle_stats.buffs.iter().cloned();

        armored + battle_buffs.sum() + self.perm_buffs.clone()
    }

    pub fn apply_status_effect(&mut self, effect: StatusEffect) {
        self.battle_stats.status_effects.insert(effect);
    }

    pub fn remove_status_effect(&mut self, effect: StatusEffect) {
        self.battle_stats.status_effects.remove(&effect);
    }

    pub fn has_status_effect(&self, effect: StatusEffect) -> bool {
        self.battle_stats.status_effects.contains(&effect)
    }

    pub fn recover_status(&mut self, rating: u8) {
        for effect in self.battle_stats.status_effects.clone() {
            if effect.rating() == rating {
                self.remove_status_effect(effect);
            }
        }
    }
}

pub struct Battlers {
    pub good_guys: Vec<Rc<RefCell<Battler>>>,
    pub bad_guys: Vec<Rc<RefCell<Battler>>>,
}
