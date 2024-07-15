use crate::prelude::*;
use godot::engine::{Sprite2D, Texture2D};
use godot::prelude::*;

fn end_ix() {
    DialogBox::singleton().bind_mut().end_interaction();
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ScriptExecutor {
    base: Base<Node>,
}

#[godot_api]
impl ScriptExecutor {
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
    fn start_ix(&self, name: String) {
        start_ix(name);
    }

    #[func]
    fn end_ix() {
        end_ix();
    }

    #[func]
    fn swap_ix(&self, name: String) {
        Self::end_ix();
        self.start_ix(name);
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

        end_ix();
        start_ix("Intro #4 >> Bed Color Picked");
    }

    #[func]
    fn debug_battle() {
        end_ix();

        World::start_battle(&EnemyID::A_NONNY_MOUSE);
    }

    #[func]
    fn debug_item(item_id: String, quantity: u32) {
        Inventory::get().borrow_mut().give_item(item_id, quantity);

        end_ix();
        start_ix("Debug Menu >> After Item");
    }
}
