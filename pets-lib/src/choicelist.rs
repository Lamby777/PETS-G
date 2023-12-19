//!
//! Helper stuff for working with choice lists
//!

use godot::prelude::*;

/// A list of concrete nodes and their associated
/// enum variants. Makes it easier to work with
/// an enum that has associated nodes for selecting
/// different options.
///
/// Incrementing past the end of the list will wrap
/// back to the start, and vice versa.
pub struct ChoiceList<Enum, T: GodotClass> {
    choices: Vec<(Enum, Gd<T>)>,
    selected: Option<usize>,
    label_tweener: Option<fn(bool, Gd<T>)>,
}

impl<Enum, T: GodotClass> Default for ChoiceList<Enum, T> {
    fn default() -> Self {
        Self {
            choices: vec![],
            selected: None,
            label_tweener: None,
        }
    }
}

impl<Enum, T: GodotClass> ChoiceList<Enum, T> {
    pub fn new(choices: impl Into<Vec<(Enum, Gd<T>)>>) -> Self {
        Self {
            choices: choices.into(),
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
            self.label_tweener.map(|f| f(false, old_node.clone()));
        }

        self.offset_by(diff);

        // tween the newly selected node
        let (_, new_node) = self.current_iv_mut().unwrap();
        self.label_tweener.map(|f| f(true, new_node.clone()));
    }

    pub fn process_input(&mut self) {
        let input = Input::singleton();

        let going_down = input.is_action_just_pressed("ui_down".into());
        let going_up = input.is_action_just_pressed("ui_up".into());
        let submitting = input.is_action_just_pressed("ui_accept".into());

        match self.current_iv_mut() {
            Some((i, _)) if submitting => {
                // self.pick_choice(*i);
            }

            _ if going_down => self.change_menu_choice(1),
            _ if going_up => self.change_menu_choice(-1),
            _ => {}
        }
    }
}
