//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::global::*;
use godot::engine::{Engine, Panel, PanelVirtual, RichTextLabel};
use godot::prelude::*;

use crate::stats::state::StatsInterface;

#[derive(GodotClass)]
#[class(base=Panel)]
struct DialogBox {
    #[base]
    node: Base<Panel>,
}

#[godot_api]
impl PanelVirtual for DialogBox {
    fn init(node: Base<Panel>) -> Self {
        Self { node }
    }

    // fn ready(&mut self) {
    fn process(&mut self, _delta: f64) {
        let mut spk_txt = self.node.get_node_as::<RichTextLabel>("SpeakerName");
        let mut msg_txt = self.node.get_node_as::<RichTextLabel>("Content");

        let mut si_object = Engine::singleton()
            .get_singleton("Stats".into())
            .unwrap()
            .cast::<StatsInterface>();

        let mut si = si_object.bind_mut();

        let content = format!("Hello, World! {}", si.get_amogus());

        spk_txt.set_text("Cherry".into());
        // msg_txt.set_text("Hello, World!".into());
        msg_txt.set_text(content.into());
        // }

        let input = Input::singleton();

        // check inputs
        // if input.is_action_pressed("battle_move_up".into()) {
        if input.is_key_pressed(Key::KEY_SPACE) {
            si.set_amogus();
        }
    }
}
