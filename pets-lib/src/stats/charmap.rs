use strum::IntoEnumIterator as _;

use crate::prelude::*;

#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct CharMap(pub Vec<Rc<RefCell<CharData>>>);

impl CharMap {
    pub fn new() -> Self {
        CharMap(Vec::new())
    }

    pub fn character(&self, ch: &PChar) -> Rc<RefCell<CharData>> {
        self.0
            .iter()
            .find(|cd| cd.borrow().id == *ch)
            .expect("Character not found")
            .clone()
    }
}

/// CharMap with all characters having the same exact stats
pub fn uniform_charmap() -> CharMap {
    PChar::iter().fold(CharMap::new(), |mut map, chname| {
        map.push(Rc::new(RefCell::new(CharData {
            id: chname,
            inherent_stats_base: InherentStats::zero(),
            ..Default::default()
        })));
        map
    })
}

/// CharMap at the start of the game
/// Most characters have unique base stats
pub fn default_charmap() -> CharMap {
    let mut map = uniform_charmap();

    // ch_unique_registry! {
    //     map,
    //
    //     ETHAN {
    //         display_name = "Ethan".to_owned(),
    //
    //         inherent_stats_base.max_hp = -10,
    //         inherent_stats_base.max_mana = Some(5),
    //         inherent_stats_base.attack = 1,
    //         inherent_stats_base.defense = 2,
    //         inherent_stats_base.speed = 4,
    //         inherent_stats_base.stability = 5,
    //         inherent_stats_base.delta = 5,
    //         inherent_stats_base.epsilon = 1,
    //         inherent_stats_base.lambda = Some(1),
    //     },
    //
    //     TERRA {
    //         display_name = "Terra".to_owned(),
    //         level = 5,
    //
    //         inherent_stats_base.max_hp = 35,
    //         inherent_stats_base.attack = 5,
    //         inherent_stats_base.defense = 4,
    //         inherent_stats_base.speed = 1,
    //         inherent_stats_base.stability = 3,
    //         inherent_stats_base.delta = 2,
    //         inherent_stats_base.epsilon = 1,
    //     },
    //
    //     SIVA {
    //         display_name = "Siva".to_owned(),
    //
    //         inherent_stats_base.max_mana = Some(10),
    //         inherent_stats_base.attack = 3,
    //         inherent_stats_base.defense = 1,
    //         inherent_stats_base.speed = 2,
    //         inherent_stats_base.stability = 5,
    //         inherent_stats_base.delta = 3,
    //         inherent_stats_base.epsilon = 1,
    //     },
    // }

    // set everyone's hp to their max
    for chardata in map.iter_mut() {
        let mut pchar = chardata.borrow_mut();
        pchar.battle_stats.hp = pchar.inherent_stats().max_hp;
    }

    map
}
