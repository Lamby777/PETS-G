//!
//! Class for overworld popout menu, where you can manage
//! inventory, eat food, etc.
//!

use godot::engine::{AnimationPlayer, Control, IPanel, InputEvent, Panel};
use godot::prelude::*;

use crate::prelude::*;

use super::inv_node::InventoryNode;

#[derive(GodotClass)]
#[class(init, base=Panel)]
pub struct WorldMenu {
    base: Base<Panel>,
    opened: bool,

    #[init(default = onready_node(&base, "Choices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,
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

        // set focus mode
        let mut choices = self.choices.bind_mut();

        choices.set_disabled(!open);
        if open {
            choices.focus_nth(0);
        }
    }

    #[func]
    pub fn open(&mut self) {
        self.open_or_close(true);
    }

    #[func]
    pub fn close(&mut self) {
        self.open_or_close(false);
    }

    #[func]
    pub fn toggle_open(&mut self) {
        self.open_or_close(!self.opened);
    }

    fn inventory_node(&self) -> Gd<InventoryNode> {
        current_scene().get_node_as("UILayer/Inventory")
    }

    fn open_inventory(&mut self) {
        self.inventory_node().bind_mut().open(true);
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        self.close();

        match choice.get_name().to_string().as_str() {
            "Inventory" => self.open_inventory(),
            "DebugQuit" => godot_tree().quit(),
            "DebugMenu" => start_ix("Debug Menu"),

            _ => unreachable!(),
        }
    }
}

#[godot_api]
impl IPanel for WorldMenu {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed".into(), callable);

        self.choices.bind_mut().disable();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            if !pcb().bind().can_move() {
                return; // you can only open the menu if you can walk
            }

            mark_input_handled(&self.base());

            self.toggle_open();
        }
    }
}
