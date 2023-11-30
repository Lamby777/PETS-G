//!
//! Dialogue system for the game's menus.
//!

pub mod autoload;
pub mod dbox;

use dialogical::prelude::*;
use indoc::indoc;

use std::sync::OnceLock;

static INTERACTIONS: OnceLock<Vec<Interaction>> = OnceLock::new();

macro_rules! packed_dialogue {
    () => {
        dialogical::deserialize(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/dg/packed.dgc"
        )))
    };
}

// load dialogues
pub fn interactions() {
    INTERACTIONS.get_or_init(|| {
        packed_dialogue!().expect(indoc! {"
            Failed to load dialogues. If you are a player,
            please report this to the developers. If you're
            a contributor, make sure the build script has
            ran properly or manually compile the interaction
            list yourself.
        "})
    });
}
