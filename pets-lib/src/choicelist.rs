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
pub struct ChoiceList<Enum, T>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    nodes: Vec<Gd<T>>,
    selected: Option<usize>,
    label_tweener: fn(bool, Gd<T>),
    on_picked: fn(Enum),
}

impl<Enum, T> Default for ChoiceList<Enum, T>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    fn default() -> Self {
        Self {
            nodes: vec![],
            selected: None,
            label_tweener: |_, _| {},
            on_picked: |_| {},
        }
    }
}

pub fn n_to_variant<Enum: TryFrom<usize>>(n: usize) -> Option<Enum> {
    Enum::try_from(n).ok()
}

impl<Enum: Copy, T: GodotClass> ChoiceList<Enum, T>
where
    Enum: TryFrom<usize>,
    T: GodotClass + Inherits<Node>,
{
    pub fn from_children_of(
        parent: Gd<Node>,
        label_tweener: fn(bool, Gd<T>),
        on_picked: fn(Enum),
    ) -> Self {
        let nodes = parent
            .get_children()
            .iter_shared()
            .map(|v| v.cast())
            .collect();

        Self {
            nodes,
            label_tweener,
            on_picked,
            ..Default::default()
        }
    }

    pub fn offset_by(&mut self, diff: i32) {
        // tween old down and new up
        if let Some((_, old_node)) = self.current_iv() {
            (self.label_tweener)(false, old_node.clone());
        }

        self.selected = Some(match self.selected {
            Some(n) => (n as i32 + diff).rem_euclid(self.nodes.len() as i32) as usize,
            None => 0,
        });

        // tween the newly selected node
        let (_, new_node) = self.current_iv().unwrap();
        (self.label_tweener)(true, new_node.clone());
    }

    /// get currently selected choice
    /// `None` if no choice is selected
    pub fn current_iv(&self) -> Option<(Enum, Gd<T>)> {
        self.selected.map(|n| {
            // can't unwrap, doesn't implement Debug
            let variant = n_to_variant(n).unwrap();
            let node = &self.nodes[n];
            (variant, node.clone())
        })
    }

    pub fn process_input(&mut self) {
        let input = Input::singleton();

        let going_down = input.is_action_just_pressed("ui_down".into());
        let going_up = input.is_action_just_pressed("ui_up".into());
        let submitting = input.is_action_just_pressed("ui_accept".into());

        match self.current_iv() {
            Some((i, _)) if submitting => {
                (self.on_picked)(i);
            }

            _ if going_down => self.offset_by(1),
            _ if going_up => self.offset_by(-1),
            _ => {}
        }
    }
}
