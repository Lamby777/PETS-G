//!
//! Helper stuff for working with choice lists
//!

use godot::prelude::*;

use crate::prelude::ListVec;

/// A list of concrete nodes and their associated
/// enum variants. Makes it easier to work with
/// an enum that has associated nodes for selecting
/// different options.
pub struct ChoiceList<Enum, T>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    listvec: ListVec<(Enum, Gd<T>)>,
}

impl<Enum: Copy, T: GodotClass> ChoiceList<Enum, T>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    pub fn process_input(&mut self) {
        fn is_pressed(name: &str) -> bool {
            Input::singleton().is_action_just_pressed(name.into())
        }

        if is_pressed("ui_down") {
            self.listvec.offset_by(1);
        } else if is_pressed("ui_up") {
            self.listvec.offset_by(-1);
        } else if is_pressed("ui_accept") {
            self.listvec.pick();
        }
    }
}
