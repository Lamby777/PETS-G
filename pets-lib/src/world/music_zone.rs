// use crate::prelude::*;
use godot::classes::{Area2D, AudioStream};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct MusicZone {
    base: Base<Area2D>,

    #[export]
    pub music: Option<Gd<AudioStream>>,
}
