use super::*;

/// Any state related to health, equipment, battling, skills, etc. that
/// should be known about a character or enemy.
///
/// This is neutral ground. Do not write player-centered code here, as
/// this struct will be used exactly the same way for enemies as well.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Battler {
    pub battle_stats: BattleStats,

    pub inherent_stats: LeveledStats,

    /// The IDs of all equipped items
    pub equipment: Equipment,
}

impl Battler {
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
        // let max_hp = self.inherent_stats().max_hp;
        // *self.hp_mut() = max_hp.min(self.hp() + amount);
        let hp = &mut self.battle_stats.hp;
        *hp = self.inherent_stats.max_hp.min(*hp + amount);
    }

    /// This should take armor, weapons, etc. into account for players.
    /// It should NOT consider in-battle buffs/debuffs.
    fn armored_stats(&self) -> LeveledStats {
        self.inherent_stats.clone() + self.equipment.offsets()
    }

    /// The final "in practice" stats of the character.
    ///
    /// Takes into account the...
    /// * Inherent stats
    /// * Equipment
    /// * Buffs
    pub fn practical_stats(&self) -> LeveledStats {
        let armored = self.armored_stats();
        let buffs = self.battle_stats.buffs.iter().cloned();

        armored + buffs.sum()
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

// Trait for stuff that both party members and enemies
// have. For example, an enemy doesn't need to have a
// "level," but it does need to have HP and status effects.
// pub trait _Battler {
//
//     fn recover_mana(&mut self, amount: IntegralStat) {
//         let max_mana = self.max_mana();
//         if let Some(max_mana) = max_mana {
//             let new_mana = max_mana.min(self.mana().unwrap_or(0) + amount);
//             *self.mana_mut().unwrap() = new_mana;
//         }
//     }
//
//
// }
