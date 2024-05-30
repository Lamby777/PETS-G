use crate::prelude::*;
use godot::engine::{AnimationPlayer, Control, IControl, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct InventoryNode {
    base: Base<Control>,

    is_open: bool,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    anim: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl InventoryNode {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn try_singleton() -> Option<Gd<InventoryNode>> {
        current_scene().try_get_node_as("%Inventory")
    }

    pub fn open(&mut self, open: bool) {
        self.is_open = open;
        self.anim.set_assigned_animation("open_inv".into());

        match open {
            true => self.anim.play(),
            false => self.anim.play_backwards(),
        }
    }
}

#[godot_api]
impl IControl for InventoryNode {
    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.is_open {
            return;
        }

        if event.is_action_pressed("menu".into()) {
            println!("Input: {:?}", event);

            self.open(false);
            mark_input_handled(&self.base());
        }
    }
}
