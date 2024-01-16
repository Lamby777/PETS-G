//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!
use crate::prelude::*;
use godot::prelude::*;

fn is_pressed(name: &str) -> bool {
    Input::singleton().is_action_just_pressed(name.into())
}

/// direction of a list's elements
enum ListDir {
    TopToBottom,
    LeftToRight,
}

/// Call a change or pick event on a listvec based
/// on the input state.
pub fn process_input<T>(list: &mut Wrapped<T>) {
    if is_pressed("ui_down") {
        list.offset_by(1);
    } else if is_pressed("ui_up") {
        list.offset_by(-1);
    } else if is_pressed("ui_accept") {
    }
}

/// Wrapping vector with a selected index
pub struct Wrapped<T> {
    elements: Vec<T>,
    selected: Option<usize>,
}

impl<T> Wrapped<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            selected: None,
        }
    }

    /// takes ownership of a vector and uses it as the list
    pub fn replace_vec(&mut self, new: Vec<T>) {
        self.elements = new;
        self.selected = None;
    }

    /// returns the currently selected index and element
    pub fn pick(&self) -> Option<(usize, &T)> {
        self.selected.map(|i| (i, &self.elements[i]))
    }

    /// Move `diff` positions forward in the list.
    /// If negative, moves backwards.
    pub fn offset_by(&mut self, diff: i32) {
        let old = self.selected.map(|i| &self.elements[i]);

        let new_i = match self.selected {
            Some(n) => (n as i32 + diff).rem_euclid(self.elements.len() as i32) as usize,
            None => 0,
        };

        self.selected = Some(new_i);
        let new = &self.elements[new_i];

        // TODO run change handler if one was set
        // if let Some(f) = self.on_changed {
        //     f(old, new);
        // }
    }
}

/// make a list from the child nodes of a parent node
/// assumes the children are in the same order as the enum variants
pub fn lv_from_children_of<Enum, T>(parent: Gd<Node>) -> Wrapped<(Enum, Gd<T>)>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    let children = parent
        .get_children()
        .iter_shared()
        .enumerate()
        .map(|(i, node)| (Enum::try_from(i).ok().unwrap(), node.cast()))
        .collect();

    Wrapped::new(children)
}

impl<T> Deref for Wrapped<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T> DerefMut for Wrapped<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

// ////////////////////////////////////////////////////////////// //
// ignore the boilerplate crap below, the compiler was being dumb //
// ////////////////////////////////////////////////////////////// //

impl<T> Default for Wrapped<T> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            selected: None,
        }
    }
}
