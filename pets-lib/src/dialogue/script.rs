use crate::common::*;
use godot::classes::{GDScript, Script};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct DialogueScript {
    base: Base<Node>,
}

#[godot_api]
impl DialogueScript {
    #[func]
    pub fn new(script: Gd<GDScript>) -> Gd<Self> {
        let mut executor = DialogueScript::new_alloc();
        executor.set_script(&script.upcast::<Script>());
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
    fn pcb(&self) -> Gd<PartyCB> {
        pcb()
    }

    // -----------------------------------------------------------------
    // ONLY DEBUG CRAP SHOULD BE PLACED BELOW!
    // Putting non-debug functions here should be treated as a code smell.
    // -----------------------------------------------------------------

    #[func]
    fn debug_item(item_id: String, quantity: i32) {
        Inventory::get().borrow_mut().give_item(item_id, quantity);
    }
}
