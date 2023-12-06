//!
//! custom class for choice labels
//!
//! this would be like 10 lines of gdscript
//! but it kept throwing errors so here we are
//! 🦀 **blazingly fast** 🦀
//!

use godot::engine::notify::ContainerNotification;
use godot::engine::{Container, IContainer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Container)]
pub struct DChoice {
    #[base]
    node: Base<Container>,
}

#[godot_api]
impl IContainer for DChoice {
    fn init(node: Base<Container>) -> Self {
        Self { node }
    }

    fn on_notification(&mut self, what: ContainerNotification) {
        let node = &mut self.node;

        if what == ContainerNotification::SortChildren {
            for c in node.get_children().iter_shared() {
                let rect = {
                    let size = node.get_size();
                    Rect2::new(Vector2::ZERO, size)
                };

                node.fit_child_in_rect(c.cast(), rect);
            }
        }
    }
}
