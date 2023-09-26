//!
//! This file is for saving/loading the game.
//!

use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;
use godot::engine::{file_access::ModeFlags, FileAccess};
use serde::{Deserialize, Serialize};

fn default_charmap() -> CharMap {
    let mut res = CharMap::new();

    let deft_stats = CharData::default();

    // add chars from registry
    for chname in PChar::ALL.iter() {
        let cloned = deft_stats.clone();
        res.insert(chname.to_string(), Rc::new(RefCell::new(cloned)));
    }

    // Ethan's special stuff
    res.entry(PChar::ETHAN.to_owned()).and_modify(|pchar| {
        let mut pchar = pchar.borrow_mut();
        pchar.base_stats.lambda = Some(1);
        pchar.base_stats.max_mana = Some(20);
    });

    // Siva's special stuff
    res.entry(PChar::SIVA.to_owned()).and_modify(|pchar| {
        let mut pchar = pchar.borrow_mut();
        pchar.base_stats.max_mana = Some(20);
    });

    res
}

/// All the data saved to one of the save file slots
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub chars: CharMap,
}

impl SaveFile {
    pub fn new_empty() -> Self {
        Self {
            chars: CharMap::new(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            chars: default_charmap(),
        }
    }

    pub fn load_from(save_slot: u8) -> Option<Self> {
        // TODO load with serde
        let path = format!("user://save{}.json", save_slot);
        let file = FileAccess::open(path.into(), ModeFlags::READ)?;
        let _content = file.get_as_text();

        todo!()
    }
}
