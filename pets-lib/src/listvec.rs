/// Abstract list of things to choose from, with listener
/// functions for when the choice is picked or changed.
///
/// Incrementing past the end of the list will wrap
/// back to the start, and vice versa.
#[derive(Default)]
pub struct ListVec<T> {
    elements: Vec<T>,
    selected: Option<usize>,

    on_changed: Option<fn(Option<&T>, &T)>,
    on_picked: Option<fn(&T)>,
}

impl<T> ListVec<T> {
    fn on_changed(&self, old: Option<&T>, new: &T) {
        if let Some(f) = self.on_changed {
            f(old, new);
        }
    }

    fn on_picked(&self, picked: &T) {
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
        self.on_changed(old, new);
    }
}
