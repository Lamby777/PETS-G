//!
//! custom class for choice labels
//!
//! this would be like 10 lines of gdscript
//! but it kept throwing errors so here we are
//! 🦀 **blazingly fast** 🦀
//!

use godot::classes::notify::ContainerNotification;
use godot::classes::{IMarginContainer, MarginContainer, RichTextLabel, Tween};
use godot::prelude::*;

use crate::common::*;
use crate::consts::dialogue::*;

#[derive(GodotClass)]
#[class(init, base=MarginContainer)]
pub struct DChoice {
    base: Base<MarginContainer>,

    #[init(node = "RichTextLabel")]
    txt_label: OnReady<Gd<RichTextLabel>>,
}

#[godot_api]
impl DChoice {
    #[func]
    pub fn set_text_tr(&mut self, text: GString) {
        self.txt_label.set_text(&tr(text.arg()));
    }

    /// tween the contained text label in/out of the window
    pub fn tween_label(&mut self, up: bool) -> Gd<Tween> {
        let label = &mut self.txt_label;
        let tw_end = if up { 0.0 } else { DBOX_CHOICE_HEIGHT };

        tween(
            label,
            "position:y",
            None,
            tw_end,
            DBOX_CHOICE_TWEEN_TIME,
            DBOX_CHOICE_TWEEN_TRANS,
        )
        .unwrap()
    }

    pub fn put_label_under(&mut self) {
        self.txt_label
            .set_position(Vector2::new(0.0, DBOX_CHOICE_HEIGHT));
    }

    /// create a new choice label with default settings
    pub fn new_container(i: usize, text: &str) -> Gd<Self> {
        let scene = load::<PackedScene>("res://scenes/dialogchoice.tscn");
        let mut dchoice = scene.instantiate_as::<Self>();

        dchoice.set_name(&format!("Choice{}", i));

        dchoice.call_deferred("set_text_tr", &[text.to_variant()]);
        // dchoice.bind_mut().set_text_tr(text.into());

        dchoice
    }
}

#[godot_api]
impl IMarginContainer for DChoice {
    fn on_notification(&mut self, what: ContainerNotification) {
        if what != ContainerNotification::SORT_CHILDREN {
            return;
        }

        let label = &self.txt_label;
        let label_size = Vector2 {
            x: label.get_size().x,
            // y: label.get_size().y + 20.0,
            y: self.base().get_size().y,
        };

        let mut base = self.base_mut();
        base.set_size(label_size);
        // base.fit_child_in_rect(label.upcast(), Rect2 {
        //     position: Vector2::ZERO,
        //     size: label_size,
        // });
    }
}
