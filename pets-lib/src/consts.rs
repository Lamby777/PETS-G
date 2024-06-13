//!
//! All the "important" constants for configuring
//! how the game works. Tinker all you want. Go nuts. :)
//!
use godot::builtin::real;
use godot::engine::tween::TransitionType;

const WAVE_BBCODE: &str = "[wave amp=100 freq=-6]";

pub mod type_aliases {
    use godot::prelude::*;
    use std::collections::HashMap;

    pub type IntegralStat = i16;
    pub type FloatStat = f32;
    pub type FnTable = HashMap<String, Callable>;
    pub type GArgs<'a, 'b> = &'a [&'b Variant];
    pub type GReturn = Result<Variant, ()>;
}

pub mod battle {
    pub const LAYER_NAME: &str = "BattleLayer";

    /// The first [this number] characters in the party
    /// are the ones that you get to use in battle.
    pub const BATTLE_PARTY_SIZE: usize = 4;

    pub const INTRO_FADE_PREDELAY: f64 = 0.5;
    pub const EFFECT_CHANCE_LIKELY: f64 = 0.8;
    pub const EFFECT_CHANCE_RARE: f64 = 0.1;
}

pub mod playercb {
    use super::*;

    // Movement physics stuff
    pub const ACCELERATION: real = 3000.0;
    pub const FRICTION: real = 2500.0;
    pub const MAX_SPEED: real = 320.0;
    pub const SPRINT_COEFFICIENT: real = 1.5;

    // Distance between party members
    pub const PERSONAL_SPACE: usize = 15;

    pub const TP_BEACON_BLACK_IN: f64 = 0.2;
    pub const TP_BEACON_BLACK_HOLD: f64 = 0.3;
    pub const TP_BEACON_BLACK_OUT: f64 = 0.2;
}

pub mod dialogue {
    use super::*;

    pub const DEFAULT_VOX: &str = "_";

    pub const TEXT_VISIBILITY_DELAY: f64 = 0.015;
    pub const PAUSE_CHAR: char = '​';
    pub const PAUSE_CHAR_DELAY: f64 = 0.1;
    pub const PUNCT_DELAY: f64 = 0.1;
    pub const WHITESPACE_DELAY: f64 = 0.03;

    pub const UI_LAYER_NAME: &str = "UILayer";
    pub const DBOX_NODE_NAME: &str = "DialogBox";

    /// distance the dialog box is from the bottom of the screen
    /// to avoid the glow effect from showing while it's not active
    pub const _DBOX_Y_BELOW_VIEWPORT: f32 = 20.0;

    pub const DBOX_CHOICE_TWEEN_TIME: f64 = choice_lists::CHOICE_TWEEN_TIME;
    pub const DBOX_CHOICE_TWEEN_TRANS: TransitionType = TransitionType::QUAD;
    pub const DBOX_CHOICE_HEIGHT: f32 = 70.0;
    pub const DBOX_CHOICE_WAVE_TIME: f64 = 0.1;

    pub const _DBOX_SELECTION_BBCODE: &str = WAVE_BBCODE;
}

pub mod choice_lists {
    use super::*;

    pub const CHOICE_TWEEN_TIME: f64 = 0.1;
    pub const CHOICE_TWEEN_TRANS: TransitionType = TransitionType::QUAD;
    pub const CHOICE_WAVE_BBCODE: &str = WAVE_BBCODE;
}
