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
    pub const DBOX_NODE_NAME: &str = "Dialog Box";
    pub const DBOX_TWEEN_TIME: f64 = 0.5;
    pub const DBOX_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;
    pub const DBOX_CHOICE_LABEL_HEIGHT: f32 = 100.0;
}

pub mod main_menu {
    use godot::engine::tween::TransitionType;

    pub const MENU_TWEEN_TIME: f64 = 0.1;
    pub const MENU_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;
    pub const MENU_WAVE_BBCODE: &str = "[wave amp=100 freq=-6]";
}
