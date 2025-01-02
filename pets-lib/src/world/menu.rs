//!
//! Class for overworld popout menu, where you can manage
//! inventory, eat food, etc.
//!

use godot::classes::{
    AnimationPlayer, Control, GDScript, IPanel, InputEvent, Panel,
    RichTextLabel,
};
use godot::prelude::*;

use crate::common::*;
use crate::dialogue::DialogueScript;

use super::inv_node::InventoryNode;

#[derive(GodotClass)]
#[class(init, base=Panel)]
pub struct WorldMenu {
    base: Base<Panel>,
    opened: bool,

    #[init(node = "Choices/ChoiceAgent")]
    choices: OnReady<Gd<ChoiceAgent>>,

    #[init(node = "AnimationPlayer")]
    anim_player: OnReady<Gd<AnimationPlayer>>,

    #[export]
    debug_menu_script: Option<Gd<GDScript>>,
}

#[godot_api]
impl WorldMenu {
    fn set_date_txt(&self, date: NaiveDate) {
        let txt = format!(
            "{} {}, {}",
            month_string_3letter(date.month()),
            date.day(),
            date.year()
        );

        self.base()
            .get_node_as::<RichTextLabel>("%DatePanel/RichTextLabel")
            .set_text(&txt);
    }

    fn open_or_close(&mut self, open: bool) {
        self.set_date_txt(si().bind().save.date);

        self.opened = open;

        self.anim_player.play_animation_forwards("open", open);

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

    fn open_inventory(&mut self) {
        InventoryNode::singleton().bind_mut().open(true);
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        self.close();

        match choice.get_name().to_string().as_str() {
            "Inventory" => self.open_inventory(),
            "DebugQuit" => godot_tree().quit(),
            "DebugMenu" => {
                let mut ds = DialogueScript::new(
                    self.debug_menu_script
                        .as_ref()
                        .expect("no debug script exported")
                        .clone(),
                );

                self.base_mut().add_child(&ds);
                ds.call("_start", &[]);
            }

            _ => unreachable!(),
        }
    }
}

#[godot_api]
impl IPanel for WorldMenu {
    fn ready(&mut self) {
        let callable = self.base().callable("on_choice_picked");
        self.choices.connect("selection_confirmed", &callable);

        self.choices.bind_mut().disable();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu") {
            if !pcb().bind().can_move() {
                return; // you can only open the menu if you can walk
            }

            mark_input_handled(&self.base());

            self.toggle_open();
        }
    }
}
