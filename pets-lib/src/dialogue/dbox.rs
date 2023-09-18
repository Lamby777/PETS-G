//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::{Engine, Panel, PanelVirtual, RichTextLabel};
use godot::prelude::*;

use crate::stats::state::StatsInterface;

#[derive(GodotClass)]
#[class(base=Panel)]
struct DialogBox {
    #[base]
    node: Base<Panel>,

    si: Gd<StatsInterface>,
    spk_txt: Gd<RichTextLabel>,
    msg_txt: Gd<RichTextLabel>,
}

#[godot_api]
impl DialogBox {
    #[func]
    fn do_draw(&mut self) {
        self.spk_txt.set_text("Cherry".into());
        self.msg_txt.set_text("Hello, World!".into());
    }
}

#[godot_api]
impl PanelVirtual for DialogBox {
    fn init(node: Base<Panel>) -> Self {
        let si = Engine::singleton()
            .get_singleton("Stats".into())
            .unwrap()
            .cast::<StatsInterface>();

        let spk_txt = node.get_node_as::<RichTextLabel>("SpeakerName");
        let msg_txt = node.get_node_as::<RichTextLabel>("Content");

        Self {
            node,
            si,
            spk_txt,
            msg_txt,
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
