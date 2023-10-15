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
pub struct DialogBox {
    #[base]
    node: Base<PanelContainer>,
    si: Gd<StatsInterface>,

    speaker: GodotString,
    content: GodotString,
}

#[godot_api]
impl DialogBox {
    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("VSplit/SpeakerName")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("VSplit/Content")
    }

    /// init new with speaker and contents provided
    #[func]
    pub fn set_txts(&mut self, speaker: GodotString, content: GodotString) {
        self.speaker = speaker;
        self.content = content;
    }

    #[func]
    pub fn do_draw(&mut self) {
        // I THINK this clone is fine, probably RC'd
        self.spk_txt().set_text(self.speaker.clone());
        self.msg_txt().set_text(self.content.clone());
    }

    #[func]
    pub fn pop_up(&mut self) {
        self.tween_into_view(true);
    }

    #[func]
    pub fn pop_down(&mut self) {
        self.tween_into_view(false);
    }

    fn tween_into_view(&mut self, up: bool) {
        let node = &mut self.node;
        let viewport_y = node.get_viewport_rect().size.y;
        let visible_y = viewport_y - node.get_size().y;

        let (tw_start, tw_end) = if up {
            (viewport_y, visible_y)
        } else {
            (visible_y, viewport_y)
        };

        let mut y_tween = node.create_tween().unwrap();
        y_tween
            .tween_property(
                node.clone().upcast(),
                "position:y".into(),
                Variant::from(tw_end),
                DBOX_TWEEN_TIME,
            )
            .unwrap()
            .from(Variant::from(tw_start))
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

            speaker: "Cherry".into(),
            content: "[wave amp=50 freq=6]Hello, World![/wave]".into(),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
