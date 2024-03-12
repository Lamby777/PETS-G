use super::*;

/// Trait for stuff that both party members and enemies
/// have. For example, an enemy doesn't need to have a
/// "level," but it does need to have HP and status effects.
pub trait Battler {
    fn hp_mut(&mut self) -> &mut IntegralStat;
    fn status_effects(&self) -> &HashSet<StatusEffect>;
    fn status_effects_mut(&mut self) -> &mut HashSet<StatusEffect>;
    fn inherent_stats(&self) -> &InherentStats;

    // These are some sensible defaults... You only really need to
    // implement the above "getters."

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
