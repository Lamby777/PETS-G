//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::tween::TransitionType;
use godot::engine::{PanelContainer, PanelContainerVirtual, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;

const DBOX_TWEEN_TIME: f64 = 1.0;
const DBOX_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;

#[derive(GodotClass)]
#[class(base=PanelContainer)]
struct DialogBox {
    #[base]
    node: Base<PanelContainer>,

    // Stat Interface reference-counted
    // This is fine to keep a ref to cuz they won't be dropped anyway
    si: Gd<StatsInterface>,
}

#[godot_api]
impl DialogBox {
    #[func]
    fn do_draw(&mut self) {
        self.spk_txt().set_text("Cherry".into());
        self.msg_txt()
            .set_text("[wave amp=50 freq=6]Hello, World![/wave]".into());
    }

    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("VSplit/SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("VSplit/Content")
    }

    fn tween_into_view(&mut self) {
        let node = &mut self.node;

        let mut y_tween = node.create_tween().unwrap();
        y_tween
            .tween_property(
                node.clone().upcast(),
                "position:y".into(),
                Variant::from(1080.0 - node.get_size().y),
                DBOX_TWEEN_TIME,
            )
            .unwrap()
            .from(Variant::from(1080))
            .unwrap()
            .set_trans(DBOX_TWEEN_TRANS);
    }
}

#[godot_api]
impl PanelContainerVirtual for DialogBox {
    fn init(node: Base<PanelContainer>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),
        }
    }

    fn ready(&mut self) {
        self.tween_into_view();
        self.do_draw();
    }
}
