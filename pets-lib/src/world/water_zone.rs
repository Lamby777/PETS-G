use godot::classes::Area2D;
use godot::prelude::*;

const DEFAULT_WATER_SPEED_MOD: real = 0.4;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct WaterZone {
    base: Base<Area2D>,

    #[export]
    #[init(val = DEFAULT_WATER_SPEED_MOD)]
    pub speed_modulation: real,
}
