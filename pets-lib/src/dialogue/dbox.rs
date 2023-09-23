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
}

#[godot_api]
impl DialogBox {
    #[func]
    fn do_draw(&mut self) {
        self.spk_txt().set_text("Cherry".into());
        self.msg_txt().set_text("Hello, World!".into());
    }

    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("SpeakerName")
    }

    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("Content")
    }
}

#[godot_api]
impl PanelVirtual for DialogBox {
    fn init(node: Base<Panel>) -> Self {
        let si = Engine::singleton()
            .get_singleton("Stats".into())
            .unwrap()
            .cast::<StatsInterface>();

        Self { node, si }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
