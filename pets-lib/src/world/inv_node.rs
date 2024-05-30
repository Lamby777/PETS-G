use crate::prelude::*;
use godot::engine::{
    AnimationPlayer, Control, HBoxContainer, IControl, InputEvent,
    RichTextLabel,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct InventoryNode {
    base: Base<Control>,

    current_index: usize,
    is_open: bool,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    anim: OnReady<Gd<AnimationPlayer>>,

    #[init(default = onready_node(&base, "%ItemsRow"))]
    row: OnReady<Gd<HBoxContainer>>,
}

#[godot_api]
impl InventoryNode {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn try_singleton() -> Option<Gd<InventoryNode>> {
        current_scene().try_get_node_as("%Inventory")
    }

    pub fn text_container(&self) -> Gd<HBoxContainer> {
        self.base().get_node_as("%InventoryText")
    }

    pub fn update_text_labels(&mut self) {
        let cont = self.text_container();
        let name_txt =
            cont.get_node_as::<RichTextLabel>("ItemName/RichTextLabel");
        let desc_txt =
            cont.get_node_as::<RichTextLabel>("ItemDesc/RichTextLabel");

        // TODO get data of item currently selected
    }

    pub fn cycle_items(&mut self, right: bool) {
        let offset = match right {
            true => 1,
            false => -1,
        };

        // let inventory_length = todo!();
        // self.current_index = (self.current_index + offset) % inventory_length;

        let animation = match right {
            true => "shift_right".into(),
            false => "shift_left".into(),
        };
        self.anim.set_assigned_animation(animation);
        self.anim.play();
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

        let is_pressed = |name: &str| event.is_action_pressed(name.into());

        if is_pressed("menu") {
            self.open(false);

            return mark_input_handled(&self.base());
        }

        if is_pressed("ui_right") || is_pressed("ui_down") {
            self.cycle_items(true);

            return mark_input_handled(&self.base());
        }

        if is_pressed("ui_left") || is_pressed("ui_up") {
            self.cycle_items(false);

            return mark_input_handled(&self.base());
        }
    }
}
