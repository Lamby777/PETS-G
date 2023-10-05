//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::{Panel, PanelVirtual, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Panel)]
struct DialogBox {
    #[base]
    node: Base<Panel>,

    // Stat Interface reference-counted
    // This is fine to keep a ref to cuz they won't be dropped anyway
    si: Gd<StatsInterface>,
}

#[godot_api]
impl DialogBox {
    #[func]
    fn do_draw(&mut self) {
        self.spk_txt().set_text("Cherry".into());
        self.msg_txt().set_text("Hello, World!".into());
    }

    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("Content")
    }
}

#[godot_api]
impl PanelVirtual for DialogBox {
    fn init(node: Base<Panel>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
