use crate::prelude::*;
use godot::engine::object::ConnectFlags;
use godot::engine::{
    AnimationPlayer, BoxContainer, Control, HBoxContainer, IControl,
    InputEvent, MarginContainer, RichTextLabel,
};

use godot::prelude::*;
use godot::tools::tr;

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
    #[func]
    pub fn singleton() -> Gd<Self> {
        World::singleton().get_node_as("%Inventory")
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn text_container(&self) -> Gd<BoxContainer> {
        self.base().get_node_as("%InventoryText")
    }

    fn item_icon(&self, index: usize) -> Gd<MarginContainer> {
        if index > self.row.get_child_count() as usize {
            panic!("Index out of bounds: {}", index);
        }

        self.row.get_node_as(format!("ItemContainer{}", index))
    }

    fn update_text_labels(&mut self) {
        let cont = self.text_container();
        let mut name_txt =
            cont.get_node_as::<RichTextLabel>("ItemName/RichTextLabel");
        let mut desc_txt =
            cont.get_node_as::<RichTextLabel>("ItemDesc/RichTextLabel");

        let inv = Inventory::get();
        let inv = inv.borrow();

        let Some((item_id, _item_count)) = inv.get_at_index(self.current_index)
        else {
            name_txt.set_text("".into());
            desc_txt.set_text("".into());
            return;
        };

        let name = tr!("ITEM_NAME_{a}", a = item_id);
        let desc = tr!("ITEM_DESC_{a}", a = item_id);
        name_txt.set_text(format!("[center]{}[/center]", name).into());
        desc_txt.set_text(format!("[center]{}[/center]", desc).into());
    }

    #[func]
    fn update_item_icons(&mut self) {
        let child_count = self.row.get_child_count();
        let inv = Inventory::get();
        let inv = inv.borrow();

        for i in 1..=child_count {
            let mut icon_cont = self.item_icon(i as usize);

            let index: i32 =
                self.current_index as i32 + i - (child_count / 2) - 1;
            if index < 0 || index >= inv.len() as i32 {
                icon_cont.call("set_texture".into(), &[Variant::nil()]);
                continue;
            }

            let item = inv.get_at_index(index as usize);

            let texture =
                item.map_or(Variant::nil(), |(id, _)| id.to_variant());
            icon_cont.call("set_texture".into(), &[texture]);
        }
    }

    #[func]
    fn on_cycle_done(&mut self, _anim_name: Variant) {
        self.update_item_icons();
        self.update_text_labels();
    }

    pub fn cycle_items(&mut self, right: bool) {
        let offset = match right {
            true => 1,
            false => -1,
        };

        let inventory_length = Inventory::get().borrow().len() as i32;
        let new_index = self.current_index as i32 + offset;

        if new_index < 0 || new_index >= inventory_length {
            return;
        }

        self.current_index = new_index as usize;

        let animation = match right {
            true => "shift_right".into(),
            false => "shift_left".into(),
        };
        self.anim.set_assigned_animation(animation);
        self.anim.play();

        // update icons once anim is over
        let callable = self.base().callable("on_cycle_done");
        self.anim
            .connect_ex("animation_finished".into(), callable)
            .flags(ConnectFlags::ONE_SHOT.ord() as u32)
            .done();
    }

    pub fn open(&mut self, open: bool) {
        self.is_open = open;
        if open {
            self.on_cycle_done(Variant::nil());
        }

        self.anim.play_animation_forwards("open_inv", open);
    }
}

#[godot_api]
impl IControl for InventoryNode {
    fn ready(&mut self) {
        self.base_mut().show();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.is_open || self.anim.is_playing() {
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
