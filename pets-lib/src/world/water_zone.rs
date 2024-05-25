use godot::engine::Area2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct WaterZone {
    base: Base<Area2D>,

    #[export]
    #[init(default = 0.7)]
    pub speed_modulation: f64,
}
