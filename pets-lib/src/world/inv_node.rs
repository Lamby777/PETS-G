use crate::prelude::*;
use godot::engine::{AnimationPlayer, Control, IControl};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct InventoryNode {
    base: Base<Control>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    anim: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl InventoryNode {
    pub fn open(&mut self, open: bool) {
        self.anim.set_assigned_animation("open_inv".into());

        match open {
            true => self.anim.play(),
            false => self.anim.play_backwards(),
        }
    }
}

#[godot_api]
impl IControl for InventoryNode {}
