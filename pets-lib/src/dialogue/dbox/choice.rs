//!
//! Stuff specifically related to dialog box choice labels
//!

use dialogical::DialogueChoice;

use godot::engine::{MarginContainer, RichTextLabel, Tween};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

use super::DialogBox;

impl DialogBox {
    fn shift_selection(&mut self, offset: i16) {
        let choice_count = self.choice_labels().len();

        let new_choice = match self.selected_choice {
            Some(v) => (v as i16 + offset).rem_euclid(choice_count as i16) as usize,
            None => 0,
        };

        self.selected_choice = Some(new_choice);
    }

    /// delete old labels and create new default ones
    pub(super) fn recreate_choice_labels(&mut self, choices: &[DialogueChoice]) {
        self.free_choice_labels();

        let mut container = self.choice_container();

        for (i, choice) in choices.iter().enumerate() {
            let margins = new_choice_label();
            let mut label = margins.get_node_as::<RichTextLabel>("Label");

            label.set_name(format!("Choice{}", i).into());
            label.set_text(choice.text.clone().into());

            container.add_child(margins.upcast());
        }
    }

    pub(super) fn tween_choices_wave(&mut self, up: bool) {
        for (i, label) in self.choice_labels().iter_shared().enumerate() {
            // we can't move the label into the closure because of
            // thread safety stuff, so just pass in the instance id
            let label_id = label.instance_id();

            let func = Callable::from_fn("choice_slide_up", move |_| {
                // get the label again using the instance id
                let Ok(label) = Gd::try_from_instance_id(label_id) else {
                    // godot_warn!("label not found");
                    return Ok(Variant::from(()));
                };

                tween_choice_label(label, up)
                    .map(|_| Variant::from(()))
                    .ok_or(())
            });

            let mut timer = godot_tree!()
                .create_timer(DBOX_CHOICE_WAVE_TIME * (i + 1) as f64)
                .unwrap();

            timer.connect("timeout".into(), func);
        }
    }
}

/// tween a label's y minimum size to grow or shrink
/// TODO inline this in tween_choices_wave
fn tween_choice_label(label: Gd<MarginContainer>, up: bool) -> Option<Gd<Tween>> {
    let tw_end = if up { DBOX_CHOICE_HEIGHT * 20.0 } else { 0.0 };

    tween(
        label.clone().upcast(),
        "custom_minimum_size:y",
        None,
        tw_end,
        DBOX_CHOICE_TWEEN_TIME,
        DBOX_CHOICE_TWEEN_TRANS,
    )
}

/// create a new choice label with default settings
fn new_choice_label() -> Gd<MarginContainer> {
    let label = load::<PackedScene>("res://scenes/dialogchoice.tscn");
    label.instantiate_as()
}
