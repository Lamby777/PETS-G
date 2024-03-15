//!
//! Dialogue system for the game's menus.
//!

mod autoload;
mod dbox;

pub use autoload::DBoxInterface;

use crate::prelude::*;
use dialogical::InteractionMap;

use std::sync::OnceLock;

static INTERACTIONS: OnceLock<InteractionMap> = OnceLock::new();

macro_rules! packed_dialogue {
    () => {
        dialogical::deserialize(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/dg/packed.dgc"
        )))
    };
}

/// Load every interaction in the game from `packed.dgc`
pub fn ix_map() -> &'static InteractionMap {
    INTERACTIONS.get_or_init(|| {
        packed_dialogue!().expect(indoc! {"
            Failed to load dialogues. If you are a player,
            please report this to the developers. If you're
            a contributor, make sure the build script has
            ran properly or manually compile the interaction
            list yourself.
        "})
    })
}
