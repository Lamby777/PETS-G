//!
//! custom class for choice labels
//!
//! this would be like 10 lines of gdscript
//! but it kept throwing errors so here we are
//! 🦀 **blazingly fast** 🦀
//!

use godot::engine::notify::ContainerNotification;
use godot::engine::{IContainer, MarginContainer, RichTextLabel};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=MarginContainer)]
pub struct DChoice {
    #[base]
    node: Base<MarginContainer>,
}

#[godot_api]
impl DChoice {
    #[func]
    pub fn set_text(&mut self, text: GString) {
        let mut label = self.base().get_node_as::<RichTextLabel>("Label");
        label.set_text(text);
    }

    /// tween the contained text label in/out of the window
    #[func]
    pub fn tween_label(&mut self, up: bool) {
        let tw_end = if up { 0.0 } else { DBOX_CHOICE_HEIGHT };

        tween(
            self.base().clone().upcast(),
            "position:y",
            None,
            tw_end,
            DBOX_CHOICE_TWEEN_TIME,
            DBOX_CHOICE_TWEEN_TRANS,
        )
        .unwrap();
    }
}

#[godot_api]
impl IContainer for DChoice {
    fn on_notification(&mut self, what: ContainerNotification) {
        if what != ContainerNotification::SortChildren {
            return;
        }

        let label = self.base().get_node_as::<RichTextLabel>("Label");
        let size = Vector2 {
            x: label.get_size().x,
            y: self.base().get_size().y,
        };

        let mut base = self.base_mut();
        base.set_size(size);
        base.fit_child_in_rect(
            label.upcast(),
            Rect2 {
                position: Vector2::ZERO,
                size,
            },
        );
    }
}
