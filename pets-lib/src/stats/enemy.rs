use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrops {
    pub item: Item,
    pub odds: u8,
}

/// All the information the game needs to know about a character
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyData {
    pub id: String,
    pub inherent_stats: InherentStats,
    pub battle_stats: BattleStats,

    pub status_effects: HashSet<StatusEffect>,
    pub equipment: Vec<Item>,
    pub drops: Vec<ItemDrops>,
}

impl EnemyData {
    pub fn from_id(id: EnemyID) -> Self {
        Self {
            id: id.to_string(),
            inherent_stats: InherentStats::default(),
            battle_stats: BattleStats::default(),
            status_effects: HashSet::new(),
            equipment: Vec::new(),
            drops: Vec::new(),
        }
    }
}

impl Battler for EnemyData {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn hp(&self) -> IntegralStat {
        self.battle_stats.hp
    }

    fn hp_mut(&mut self) -> &mut IntegralStat {
        &mut self.battle_stats.hp
    }

    fn status_effects(&self) -> &HashSet<StatusEffect> {
        &self.status_effects
    }

    fn status_effects_mut(&mut self) -> &mut HashSet<StatusEffect> {
        &mut self.status_effects
    }

    fn inherent_stats(&self) -> &InherentStats {
        &self.inherent_stats
    }

    fn equipment(&self) -> &[Item] {
        &self.equipment
    }

    fn buffs_list(&self) -> &[InherentStats] {
        &self.battle_stats.buffs
    }

    fn mana(&self) -> Option<IntegralStat> {
        self.battle_stats.mana
    }

    fn mana_mut(&mut self) -> Option<&mut IntegralStat> {
        self.battle_stats.mana.as_mut()
    }
}
