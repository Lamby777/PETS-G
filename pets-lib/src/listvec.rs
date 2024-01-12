//!
//! Helper crap for dealing with user-facing
//! lists of stuff
//!
use godot::prelude::*;

/// Abstract list of things to choose from, with listener
/// functions for when the choice is picked or changed.
///
/// Incrementing past the end of the list will wrap
/// back to the start, and vice versa.
pub struct ListVec<T> {
    elements: Vec<T>,
    selected: Option<usize>,

    on_changed: Option<fn(Option<&T>, &T)>,
    on_picked: Option<fn(&T)>,
}

impl<T> ListVec<T> {
    pub fn new(
        elements: Vec<T>,
        on_changed: Option<fn(Option<&T>, &T)>,
        on_picked: Option<fn(&T)>,
    ) -> Self {
        Self {
            elements,
            selected: None,
            on_changed,
            on_picked,
        }
    }

    pub fn selected_pair(&self) -> Option<(usize, &T)> {
        self.selected.map(|i| (i, &self.elements[i]))
    }

    /// Call the pick handler on the currently selected element.
    pub fn pick(&self) {
        let picked = self
            .selected
            .map(|i| &self.elements[i])
            .expect("Called `pick` on choice out of bounds!");

        // Calling `pick` on a bad index should always error,
        // even if no `on_picked` function is set. This is just
        // to make sure the dev knows they screwed up.

        if let Some(f) = self.on_picked {
            f(picked);
        }
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

        // run change handler if one was set
        if let Some(f) = self.on_changed {
            f(old, new);
        }
    }
}

/// Call a change or pick event on a listvec based
/// on the input state.
pub fn process_input<T>(lv: &mut ListVec<T>) {
    fn is_pressed(name: &str) -> bool {
        Input::singleton().is_action_just_pressed(name.into())
    }

    if is_pressed("ui_down") {
        lv.offset_by(1);
    } else if is_pressed("ui_up") {
        lv.offset_by(-1);
    } else if is_pressed("ui_accept") {
        lv.pick();
    }
}

/// For when you want to map enum values to concrete nodes
pub struct ChoiceList<Enum, T>(ListVec<(Enum, Gd<T>)>)
where
    T: GodotClass + Inherits<Node>;

impl<Enum, T> ChoiceList<Enum, T>
where
    T: GodotClass + Inherits<Node>,
    Enum: TryFrom<usize>,
{
    pub fn inner_mut(&mut self) -> &mut ListVec<(Enum, Gd<T>)> {
        &mut self.0
    }

    /// make a list from the child nodes of a parent node
    /// assumes the children are in the same order as the enum variants
    pub fn from_children_of(
        parent: Gd<Node>,

        // what the fuck
        // TODO https://github.com/rust-lang/rust/issues/8995
        on_change: Option<fn(Option<&(Enum, Gd<T>)>, &(Enum, Gd<T>))>,
        on_picked: Option<fn(&(Enum, Gd<T>))>,
    ) -> Self {
        let children = parent
            .get_children()
            .iter_shared()
            .enumerate()
            .map(|(i, node)| (Enum::try_from(i).ok().unwrap(), node.cast()))
            .collect();

        let lv = ListVec::new(children, on_change, on_picked);
        Self(lv)
    }
}

// ////////////////////////////////////////////////////////////// //
//                                                                //
// ignore the boilerplate crap below, the compiler was being dumb //
//                                                                //
// ////////////////////////////////////////////////////////////// //

impl<T> Default for ListVec<T> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            selected: None,
            on_changed: None,
            on_picked: None,
        }
    }
}

impl<Enum, T> Default for ChoiceList<Enum, T>
where
    T: GodotClass + Inherits<Node>,
{
    fn default() -> Self {
        Self(ListVec::default())
    }
}
