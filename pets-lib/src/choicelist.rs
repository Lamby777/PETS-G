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
}

impl<Enum, T: GodotClass> Default for ChoiceList<Enum, T> {
    fn default() -> Self {
        Self {
            choices: vec![],
            selected: None,
        }
    }
}

impl<Enum, T: GodotClass> ChoiceList<Enum, T> {
    pub fn new(choices: impl Into<Vec<(Enum, Gd<T>)>>) -> Self {
        Self {
            choices: choices.into(),
            selected: None,
        }
    }

    pub fn offset_by(&mut self, diff: i32) {
        self.selected = Some(match self.selected {
            Some(n) => ((n as i32 + diff) as usize).rem_euclid(self.choices.len()),
            None => 0,
        });
    }

    /// get currently selected choice
    pub fn current_iv_mut(&self) -> Option<&(Enum, Gd<T>)> {
        self.selected.map(|n| &self.choices[n])
    }
}
