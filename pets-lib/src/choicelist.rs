//!
//! Helper stuff for working with choice lists
//!

use godot::prelude::*;

type TweenFn<T> = fn(bool, Gd<T>);
type PickHandler<Owner, Enum> = fn(&mut Owner, Enum);

/// A list of concrete nodes and their associated
/// enum variants. Makes it easier to work with
/// an enum that has associated nodes for selecting
/// different options.
///
/// Incrementing past the end of the list will wrap
/// back to the start, and vice versa.
pub struct ChoiceList<Enum, T: GodotClass, Owner> {
    choices: Vec<(Enum, Gd<T>)>,
    selected: Option<usize>,
    label_tweener: TweenFn<T>,
    on_picked: PickHandler<Owner, Enum>,
}

impl<Enum, T: GodotClass, Owner> Default for ChoiceList<Enum, T, Owner> {
    fn default() -> Self {
        Self {
            choices: vec![],
            selected: None,
            label_tweener: |_, _| {},
            on_picked: |_, _| {},
        }
    }
}

impl<Enum: Copy, T: GodotClass, Owner> ChoiceList<Enum, T, Owner> {
    pub fn new(
        choices: impl Into<Vec<(Enum, Gd<T>)>>,
        label_tweener: TweenFn<T>,
        on_picked: PickHandler<Owner, Enum>,
    ) -> Self {
        Self {
            choices: choices.into(),
            label_tweener,
            on_picked,
            ..Default::default()
        }
    }

    pub fn offset_by(&mut self, diff: i32) {
        self.selected = Some(match self.selected {
            Some(n) => ((n as i32 + diff) as usize).rem_euclid(self.choices.len()),
            None => 0,
        });
    }

    /// get currently selected choice
    /// `None` if no choice is selected
    pub fn current_iv_mut(&self) -> Option<&(Enum, Gd<T>)> {
        self.selected.map(|n| &self.choices[n])
    }

    fn change_menu_choice(&mut self, diff: i32) {
        // tween old down and new up
        if let Some((_, old_node)) = self.current_iv_mut() {
            (self.label_tweener)(false, old_node.clone());
        }

        self.offset_by(diff);

        // tween the newly selected node
        let (_, new_node) = self.current_iv_mut().unwrap();
        (self.label_tweener)(true, new_node.clone());
    }

    pub fn process_input(&mut self) {
        let input = Input::singleton();

        let going_down = input.is_action_just_pressed("ui_down".into());
        let going_up = input.is_action_just_pressed("ui_up".into());
        let submitting = input.is_action_just_pressed("ui_accept".into());

        match self.current_iv_mut() {
            Some((i, _)) if submitting => {
                // (self.on_picked)(*i);
            }

            _ if going_down => self.change_menu_choice(1),
            _ if going_up => self.change_menu_choice(-1),
            _ => {}
        }
    }
}
