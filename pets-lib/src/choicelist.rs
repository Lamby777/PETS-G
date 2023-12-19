//!
//! Helper stuff for working with choice lists
//!

use godot::prelude::*;

/// A list of concrete nodes and their associated
/// enum variants. Makes it easier to work with
/// an enum that has associated nodes for selecting
/// different options.
pub struct ChoiceNodes<Enum, T: GodotClass>(pub ChoiceList<(Enum, Gd<T>)>);

impl<Enum, T: GodotClass> ChoiceNodes<Enum, T> {
    //
}

/// An abstract representation of a list of choices.
/// Incrementing past the end of the list will wrap
/// back to the start, and vice versa.
pub struct ChoiceList<T> {
    choices: Vec<T>,
    selected: Option<usize>,
}

impl<T> Default for ChoiceList<T> {
    fn default() -> Self {
        Self {
            choices: vec![],
            selected: None,
        }
    }
}

impl<T> ChoiceList<T> {
    pub fn new(choices: impl Into<Vec<T>>) -> Self {
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
    pub fn current_iv_mut(&mut self) -> Option<(usize, &mut T)> {
        self.selected.map(|n| (n, &mut self.choices[n]))
    }
}
