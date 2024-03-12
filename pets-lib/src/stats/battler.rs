use super::*;

/// Trait for stuff that both party members and enemies
/// have. For example, an enemy doesn't need to have a
/// "level," but it does need to have HP and status effects.
pub trait Battler {
    fn hp_mut(&mut self) -> &mut IntegralStat;
    fn status_effects(&self) -> &HashSet<StatusEffect>;
    fn status_effects_mut(&mut self) -> &mut HashSet<StatusEffect>;

    /// This should take armor, weapons, etc. into account for players.
    /// It should NOT consider in-battle buffs/debuffs.
    fn inherent_stats(&self) -> &InherentStats;

    /// This should return a reference to the list of currently active (de)buffs
    fn buffs_list(&self) -> &[InherentStats];

    //
    // Below are some sensible defaults... You only really need to
    // implement the above "getters" to make the rest work.
    //

    /// The final "in practice" stats of the character.
    ///
    /// Takes into account the...
    /// * Inherent stats
    /// * Equipment
    /// * Buffs
    fn practical_stats(&self) -> InherentStats {
        let inherent = self.inherent_stats().clone();

        self.buffs_list()
            .iter()
            .cloned()
            .fold(inherent, |acc, buff| acc + buff)
    }

    fn max_hp(&self) -> IntegralStat {
        self.inherent_stats().max_hp
    }

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
