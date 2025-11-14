// pub struct Battler {
//     pub level: Option<IntegralStat>,
//     pub battle_stats: BattleStats,
//     pub perm_buffs: LeveledStats,
//     pub equipment: Equipment,
// }

{
  level: null,
  battle_stats: {
    hp: 30,
    mana: null,
    energy: 1,
    buffs: [],
    status_effects: [],
  },
  perm_buffs: {
    max_hp: 0,
    max_energy: 0,
    attack: 0,
    defense: 0,
    speed: 0,
    stability: 0,
    delta: 0,
    epsilon: 0,
    lambda: 0,
    max_mana: 0,
  },
  equipment: {
    head: null,
    body: null,
    weapon: null,
    accessories: [null, null, null],
  },
}
