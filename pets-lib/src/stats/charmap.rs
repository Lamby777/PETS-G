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

    // set everyone's hp to their max
    for chardata in map.iter_mut() {
        let mut pchar = chardata.borrow_mut();
        pchar.battle_stats.hp = pchar.inherent_stats().max_hp;
    }

    map
}
