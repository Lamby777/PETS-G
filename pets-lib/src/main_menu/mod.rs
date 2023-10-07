//!
//! Main menu scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TitleScreen {
    #[base]
    node: Base<Node2D>,
    si: Gd<StatsInterface>,
}

#[godot_api]
impl TitleScreen {
    // #[func]
    // fn do_draw(&mut self) {
    //     self.spk_txt().set_text("Cherry".into());
    // }

    // /// Get the speaker name label
    // fn spk_txt(&self) -> Gd<RichTextLabel> {
    //     self.node.get_node_as::<RichTextLabel>("SpeakerName")
    // }
}

#[godot_api]
impl Node2DVirtual for TitleScreen {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),
        }
    }

    fn ready(&mut self) {
        // self.do_draw();
    }
}
