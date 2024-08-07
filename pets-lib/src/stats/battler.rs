use super::*;

/// Trait for stuff that both party members and enemies
/// have. For example, an enemy doesn't need to have a
/// "level," but it does need to have HP and status effects.
pub trait Battler {
    fn id(&self) -> String;

    fn hp(&self) -> IntegralStat;
    fn hp_mut(&mut self) -> &mut IntegralStat;

    fn mana(&self) -> Option<IntegralStat>;
    fn mana_mut(&mut self) -> Option<&mut IntegralStat>;

    fn status_effects(&self) -> &HashSet<StatusEffect>;
    fn status_effects_mut(&mut self) -> &mut HashSet<StatusEffect>;

    /// This returns a copy of the inherent stats
    fn inherent_stats(&self) -> InherentStats;
    fn equipment(&self) -> &[Item];

    /// This should return a reference to the list of currently active (de)buffs
    fn buffs_list(&self) -> &[InherentStats];

    //
    // Below are some sensible defaults... You only really need to
    // implement the above "getters" to make the rest work.
    //

    /// This should take armor, weapons, etc. into account for players.
    /// It should NOT consider in-battle buffs/debuffs.
    fn armored_stats(&self) -> InherentStats {
        let inherent = self.inherent_stats().clone();

        // get all offsets from each item that has one
        let equips = self.equipment();
        let offsets = equips.offsets();

        // ... and sum them up
        inherent + offsets.cloned().sum()
    }

    /// The final "in practice" stats of the character.
    ///
    /// Takes into account the...
    /// * Inherent stats
    /// * Equipment
    /// * Buffs
    fn practical_stats(&self) -> InherentStats {
        let armored = self.armored_stats();
        let buffs = self.buffs_list().iter().cloned();

        armored + buffs.sum()
    }

    fn max_hp(&self) -> IntegralStat {
        self.practical_stats().max_hp
    }

    fn max_mana(&self) -> Option<IntegralStat> {
        self.practical_stats().max_mana
    }

    fn recover_hp(&mut self, amount: IntegralStat) {
        let max_hp = self.max_hp();
        let new_hp = max_hp.min(self.hp() + amount);
        *self.hp_mut() = new_hp;
    }

    fn recover_mana(&mut self, amount: IntegralStat) {
        let max_mana = self.max_mana();
        if let Some(max_mana) = max_mana {
            let new_mana = max_mana.min(self.mana().unwrap_or(0) + amount);
            *self.mana_mut().unwrap() = new_mana;
        }
    }

    fn recover_status(&mut self, rating: u8) {
        for effect in self.status_effects().clone() {
            if effect.rating() == rating {
                self.remove_status_effect(effect);
            }
        }
    }

    /// Subtract damage count from the character's HP, stopping at 0.
    /// Returns the new HP.
    fn take_damage(&mut self, damage: IntegralStat) -> IntegralStat {
        let new_hp = 0.max(self.hp() - damage);
        *self.hp_mut() = new_hp;

        new_hp
    }

    /// Add back HP to the character
    ///
    /// Saturated at the character's max HP
    fn heal(&mut self, amount: IntegralStat) {
        let max_hp = self.inherent_stats().max_hp;
        *self.hp_mut() = max_hp.min(self.hp() + amount);
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

pub struct Battlers {
    pub good_guys: Vec<Rc<RefCell<dyn Battler>>>,
    pub bad_guys: Vec<Rc<RefCell<dyn Battler>>>,
}
