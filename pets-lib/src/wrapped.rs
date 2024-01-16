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
pub enum ListDir {
    TopToBottom,
    LeftToRight,
}

impl ListDir {
    pub fn ui_next(&self) -> &str {
        use ListDir::*;
        match self {
            TopToBottom => "ui_down",
            LeftToRight => "ui_right",
        }
    }

    pub fn ui_prev(&self) -> &str {
        use ListDir::*;
        match self {
            TopToBottom => "ui_up",
            LeftToRight => "ui_left",
        }
    }
}

pub enum ListOperation {
    Walk(bool),
    Pick,
}

/// Convert user input into list navigation
pub fn process_input(dir: ListDir) -> Option<ListOperation> {
    use ListOperation::*;

    Some(match () {
        _ if is_pressed(dir.ui_next()) => Walk(false),
        _ if is_pressed(dir.ui_prev()) => Walk(true),
        _ if is_pressed("ui_accept") => Pick,
        _ => return None,
    })
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

    pub fn next(&mut self) {
        self.walk(false);
    }

    pub fn prev(&mut self) {
        self.walk(true);
    }

    pub fn walk(&mut self, backwards: bool) {
        let diff = if backwards { -1 } else { 1 };
        self.selected = Some(match self.selected {
            Some(n) => (n as i32 + diff).rem_euclid(self.elements.len() as i32) as usize,
            None => 0,
        });
    }
}

/// make a list from the child nodes of a parent node
/// assumes the children are in the same order as the enum variants
pub fn from_children_of<Enum, T>(parent: Gd<Node>) -> Wrapped<(Enum, Gd<T>)>
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
