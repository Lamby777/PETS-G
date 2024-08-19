use crate::common::*;
use godot::classes::{GDScript, Sprite2D, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct DialogueScriptBase {
    base: Base<Node>,
}

#[godot_api]
impl DialogueScriptBase {
    #[func]
    pub fn new(script: Gd<GDScript>) -> Gd<Self> {
        let mut executor = DialogueScriptBase::new_alloc();
        executor.set_script(script.to_variant());
        executor
    }

    /// This is the entry point for all dialogue scripts.
    #[allow(non_snake_case)]
    #[func(virtual)]
    pub fn _start(&mut self) {}

    // Convenience stuff to prevent long lines in embedded dialogue scripts
    #[func]
    fn dbox(&self) -> Gd<DialogBox> {
        DialogBox::singleton()
    }

    #[func]
    fn pcb(&self) -> Gd<PlayerCB> {
        pcb()
    }

    #[func]
    fn end() {
        DialogBox::singleton().bind_mut().end();
    }

    // -----------------------------------------------------------------

    #[func]
    fn set_ethan_bed_color(color: String) {
        let texture = load::<Texture2D>(format!(
            "res://assets/textures/builds/furniture/beds/bed_{color}.png"
        ));

        si().bind_mut().save.bed_color = color;

        let mut bed =
            World::room().get_node_as::<Sprite2D>("%EthanBed/Sprite2D");
        bed.callv("set_texture".into(), varray![texture]);
    }

    #[func]
    fn debug_item(item_id: String, quantity: u32) {
        Inventory::get().borrow_mut().give_item(item_id, quantity);
        // start_ix("Debug Menu >> After Item");
    }
}
