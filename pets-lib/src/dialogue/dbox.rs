//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::{Panel, PanelVirtual, RichTextLabel};
use godot::prelude::*;

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

    fn ready(&mut self) {
        let node = &mut self.node;

        let mut spk_txt = node.get_node_as::<RichTextLabel>("SpeakerName");
        let mut msg_txt = node.get_node_as::<RichTextLabel>("Content");

        spk_txt.set_text("Cherry".into());
        msg_txt.set_text("Hello, World!".into());
    }
}
