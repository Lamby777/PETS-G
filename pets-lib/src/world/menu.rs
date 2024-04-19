//!
//! Class for overworld popout menu, where you can manage
//! inventory, eat food, etc.
//!

use godot::engine::{
    AnimationPlayer, IPanel, InputEvent, Panel, RichTextLabel,
};
use godot::prelude::*;
use num_enum::TryFromPrimitive;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(usize)]
enum Choice {
    Inventory,
    DebugQuit, // TODO use this instead of shift+q
}

#[derive(GodotClass)]
#[class(init, base=Panel)]
pub struct WorldMenu {
    base: Base<Panel>,
    choices: Wrapped<(Choice, Gd<RichTextLabel>)>,

    opened: bool,
}

#[godot_api]
impl WorldMenu {
    fn anim_player(&self) -> Gd<AnimationPlayer> {
        self.base().get_node_as("AnimationPlayer")
    }

    fn open_or_close(&mut self, open: bool) {
        self.opened = open;

        let mut anim = self.anim_player();
        anim.set_assigned_animation("open".into());

        if open {
            anim.play();
        } else {
            anim.play_backwards()
        }
    }
}

#[godot_api]
impl IPanel for WorldMenu {
    fn process(&mut self, _delta: f64) {
        //
    }

    fn ready(&mut self) {
        let cont = self.base().get_node_as("Choices");
        self.choices = Wrapped::from_children_of(cont);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            mark_input_handled(&self.base());

            self.open_or_close(!self.opened);
        }
    }
}
