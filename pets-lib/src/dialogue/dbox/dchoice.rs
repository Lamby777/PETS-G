//!
//! custom class for choice labels
//!
//! this would be like 10 lines of gdscript
//! but it kept throwing errors so here we are
//! 🦀 **blazingly fast** 🦀
//!

use godot::engine::notify::ContainerNotification;
use godot::engine::{Container, IContainer, RichTextLabel};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Container)]
pub struct DChoice {
    #[base]
    node: Base<Container>,
}

#[godot_api]
impl DChoice {
    #[func]
    pub fn set_text(&mut self, text: GString) {
        let mut label = self.node.get_node_as::<RichTextLabel>("Label");
        label.set_text(text);
    }
}

#[godot_api]
impl IContainer for DChoice {
    fn on_notification(&mut self, what: ContainerNotification) {
        if what != ContainerNotification::SortChildren {
            return;
        }

        let label = self.node.get_node_as::<RichTextLabel>("Label");
        let size = Vector2 {
            x: label.get_size().x,
            y: self.node.get_size().y,
        };

        // self.node.set_size(size);

        let rect = Rect2::new(Vector2::ZERO, size);
        self.node.fit_child_in_rect(label.upcast(), rect);
    }
}
