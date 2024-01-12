use godot::engine::Input;

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

    pub fn inner(&self) -> &[T] {
        &self.elements
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
