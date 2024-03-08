//!
//! All the "important" constants for configuring
//! how the game works. Tinker all you want. Go nuts. :)
//!
use godot::builtin::real;
use godot::engine::tween::TransitionType;

pub mod battle {
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
}

pub mod dialogue {
    use super::*;

    pub const NARRATOR_DISPLAYNAME: &str = "";
    pub const UNKNOWN_DISPLAYNAME: &str = "???";
    pub const DEFAULT_VOX: &str = "_";

    pub const UI_LAYER_NAME: &str = "UILayer";
    pub const DBOX_NODE_NAME: &str = "DialogBox";
    pub const DBOX_TWEEN_TIME: f64 = 0.5;
    pub const DBOX_TWEEN_TRANS: TransitionType = TransitionType::QUAD;

    /// distance the dialog box is from the bottom of the screen
    /// to avoid the glow effect from showing while it's not active
    pub const DBOX_Y_BELOW_VIEWPORT: f32 = 20.0;

    pub const DBOX_CHOICE_TWEEN_TIME: f64 = main_menu::MENU_TWEEN_TIME;
    pub const DBOX_CHOICE_TWEEN_TRANS: TransitionType = DBOX_TWEEN_TRANS;
    pub const DBOX_CHOICE_HEIGHT: f32 = 60.0;
    pub const DBOX_CHOICE_WAVE_TIME: f64 = 0.1;

    pub const DBOX_SELECTION_BBCODE: &str = "[wave amp=100 freq=-6]";
}

pub mod main_menu {
    use super::*;

    pub const MENU_TWEEN_TIME: f64 = 0.1;
    pub const MENU_TWEEN_TRANS: TransitionType = TransitionType::QUAD;
    pub const MENU_WAVE_BBCODE: &str = dialogue::DBOX_SELECTION_BBCODE;
}
