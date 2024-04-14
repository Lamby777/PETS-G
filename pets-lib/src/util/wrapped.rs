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
pub enum ListDirection {
    TopToBottom,
    LeftToRight,
}

impl ListDirection {
    pub fn ui_next(&self) -> &str {
        use ListDirection::*;
        match self {
            TopToBottom => "ui_down",
            LeftToRight => "ui_right",
        }
    }

    pub fn ui_prev(&self) -> &str {
        use ListDirection::*;
        match self {
            TopToBottom => "ui_up",
            LeftToRight => "ui_left",
        }
    }
}

pub enum ListOperation<'a, T> {
    /// Reference to the old and new elements
    Walk(Option<&'a T>, &'a T),

    /// Index of and a reference to the picked element
    Pick(usize, &'a T),

    /// Either no input or tried to confirm on no selection
    Nothing,
}

/// Convert user input into list navigation
pub fn process_input<T: Clone>(
    wrap: &mut Wrapped<T>,
    dir: ListDirection,
) -> ListOperation<T> {
    use ListOperation::*;

    let is_reverse = match wrap.selected {
        Some(i) if is_pressed("ui_accept") => return Pick(i, &wrap[i]),
        _ if is_pressed(dir.ui_next()) => false,
        _ if is_pressed(dir.ui_prev()) => true,
        _ => return Nothing,
    };

    // hasn't returned yet, so we're walking

    let old_i = wrap.selected;
    wrap.walk(is_reverse);

    // index the vector after calling `walk` (avoids borrow check issues)
    let old = old_i.map(|i| &wrap[i]);

    // guaranteed to be `Some` because we just called `walk`
    let new = wrap.pick().unwrap();

    Walk(old, new)
}

#[derive(Deref, DerefMut)]
/// Wrapping vector with a selected index
pub struct Wrapped<T> {
    #[target]
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
    pub fn _pick_iv(&self) -> Option<(usize, &T)> {
        self.selected.map(|i| (i, &self.elements[i]))
    }

    /// returns the currently selected element
    pub fn pick(&self) -> Option<&T> {
        self.selected.map(|i| &self.elements[i])
    }

    /// move selection forwards or backwards
    pub fn walk(&mut self, backwards: bool) {
        let diff = if backwards { -1 } else { 1 };
        self.selected = Some(match self.selected {
            Some(n) => (n as i32 + diff).rem_euclid(self.elements.len() as i32)
                as usize,
            None => 0,
        });
    }
}

impl<T> Wrapped<T>
where
    T: GodotClass,
{
    /// make a list from the child nodes of a parent node
    /// assumes the children are in the same order as the enum variants
    pub fn from_children_of<Enum>(parent: Gd<Node>) -> Wrapped<(Enum, Gd<T>)>
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
