//!
//! All the "important" constants for configuring
//! how the game works. Tinker all you want. Go nuts. :)
//!

pub mod playercb {
    // Movement physics stuff
    pub const ACCELERATION: f64 = 3000.0;
    pub const FRICTION: f64 = 2500.0;
    pub const MAX_SPEED: f64 = 320.0;

    // Distance between party members
    pub const PERSONAL_SPACE: u16 = 15;
}

pub mod dialogue {
    use godot::engine::tween::TransitionType;

    pub const NARRATOR_DISPLAYNAME: &str = "";
    pub const UNKNOWN_DISPLAYNAME: &str = "???";
    pub const DEFAULT_VOX: &str = "_";

    pub const UI_LAYER_NAME: &str = "UILayer";
    pub const DBOX_NODE_NAME: &str = "DialogBox";
    pub const DBOX_TWEEN_TIME: f64 = 0.5;
    pub const DBOX_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;

    /// distance the dialog box is from the bottom of the screen
    /// to avoid the glow effect from showing while it's not active
    pub const DBOX_Y_BELOW_VIEWPORT: f32 = 20.0;

    // pub const DBOX_CHOICE_TWEEN_TIME: f64 = DBOX_TWEEN_TIME;
    // pub const DBOX_CHOICE_TWEEN_TRANS: TransitionType = DBOX_TWEEN_TRANS;
    pub const DBOX_CHOICE_HEIGHT: f32 = 60.0;
    pub const DBOX_CHOICE_WAVE_TIME: f64 = 0.1;
}

pub mod main_menu {
    use godot::engine::tween::TransitionType;

    pub const MENU_TWEEN_TIME: f64 = 0.1;
    pub const MENU_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;
    pub const MENU_WAVE_BBCODE: &str = "[wave amp=100 freq=-6]";
}
