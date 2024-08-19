use crate::common::*;
use godot::classes::GDScript;
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
    // ONLY DEBUG CRAP SHOULD BE PLACED BELOW!
    // Putting non-debug functions here should be treated as a code smell.
    // -----------------------------------------------------------------

    #[func]
    fn debug_item(item_id: String, quantity: u32) {
        Inventory::get().borrow_mut().give_item(item_id, quantity);
    }
}
