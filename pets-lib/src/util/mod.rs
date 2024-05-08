//
//!
//! General-purpose Godot/Rust helper stuff
//!

pub mod choices;
pub mod limiq;
pub mod singleton;

use godot::engine::tween::TransitionType;
use godot::engine::{Engine, RichTextLabel, Theme, Tween};
use godot::prelude::*;

pub fn mark_input_handled<T>(node: &Gd<T>)
where
    T: Inherits<Node>,
{
    node.upcast_ref()
        .get_viewport()
        .unwrap()
        .set_input_as_handled();
}

/// Recursively get all children of a node that are of a certain type.
/// Returns a vector. Use `subchildren_of_type_array` for a godot array.
///
/// bugfix later: it won't find children of nodes that are the correct type
pub fn subchildren_of_type<T>(parent: Gd<Node>) -> Vec<Gd<T>>
where
    T: GodotClass + Inherits<Node>,
{
    let mut res = vec![];

    for node in parent.get_children().iter_shared() {
        let Ok(node) = node.clone().try_cast::<T>() else {
            let children = subchildren_of_type::<T>(node);
            res.extend(children);
            continue;
        };

        res.push(node);
    }

    res
}

pub trait Vector2Ext {
    fn to_tuple(&self) -> (f32, f32);
}

impl Vector2Ext for Vector2 {
    /// Convert the godot Vector2 into a tuple of x and y.
    fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

/// helper function to load nodes into `OnReady` fields
/// adapted from bromeon's answer on the gdext discord
pub fn onready_node<O, T>(
    this: &Base<O>,
    path: impl Into<NodePath> + 'static,
) -> OnReady<Gd<T>>
where
    T: Inherits<Node>,
    O: Inherits<Node>,
{
    let self_obj = this.to_gd();
    OnReady::new(move || self_obj.upcast().get_node_as(path))
}

/// takes a bbcode string and prepends or removes it from the label text
pub fn bbcode_toggle(mut node: Gd<RichTextLabel>, bbcode: &str, active: bool) {
    let old_text = node.get_text().to_string();
    let new_text = prefix_mod(&old_text, bbcode, active);

    node.set_text(new_text.into());
}

/// adds `prefix` to the start of `target` if `active` is true...
/// otherwise removes the first `prefix.len()` characters
///
/// panics if `target` is shorter than `prefix`.
/// you also need to make sure you don't call it with `false`
/// if the prefix isn't already there. be careful with this function...
pub fn prefix_mod(target: &str, prefix: &str, active: bool) -> String {
    if active {
        format!("{}{}", prefix, target)
    } else {
        let st: String = target.into();
        st[prefix.len()..].to_owned()
    }
}

/// shorthand to do some tweeneroonies :3
pub fn tween<NP, V>(
    mut node: Gd<Node>,
    property: NP,
    start_value: Option<V>,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    NP: Into<NodePath>,
    V: ToGodot,
{
    let res: Option<_> = try {
        let mut tween = node.create_tween()?;

        let mut property = tween
            .tween_property(
                node.clone().upcast(),
                property.into(),
                end_value.to_variant(),
                time,
            )?
            .set_trans(trans)?;

        if let Some(start_value) = start_value {
            property.from(start_value.to_variant())?;
        }

        tween
    };

    res.ok_or(())
}

pub fn default_theme() -> Gd<Theme> {
    load("res://themes/theme_deft.tres")
}

pub fn godot_tree() -> Gd<SceneTree> {
    Engine::singleton().get_main_loop().unwrap().cast()
}

pub fn current_scene() -> Gd<Node> {
    godot_tree().get_current_scene().unwrap()
}

pub use crate::change_scene;
#[macro_export]
macro_rules! change_scene {
    ($scene:expr) => {
        godot_tree().change_scene_to_file(
            concat!("res://scenes/", $scene, ".tscn").into(),
        )
    };
}

/// List elements separated by commas, but put a word
/// like "or" or "and" before the last one to sound less robotic.
///
/// # Edge Cases it handles...
/// - no commas if there are only two elements
/// - nothing happens if there's only one element
/// - returns None if the list is empty
///
/// # Prepend Stuff
/// Prepends `prepend_word` if there's only one element.
/// This is good for words like "only" or "just".
///
/// Song:    "Conjunction Junction, what's your function?"
/// P/E/T/S: "This one right here, of course!"
pub fn join_words<I, T>(
    mut iter: I,
    conjunction: &str,
    prepend_word: Option<&str>,
) -> Option<String>
where
    I: Iterator<Item = T> + Clone,
    T: ToString,
{
    let len = iter.clone().count();
    let mut next = || iter.next().unwrap().to_string();

    Some(match len {
        0 => return None,
        1 => match prepend_word {
            Some(pre) => format!("{} {}", pre, next()),
            None => next(),
        },

        2 => {
            format!("{} {} {}", next(), conjunction, next())
        }

        _ => {
            let iter = iter.map(|x| x.to_string());
            let first_part = iter.clone().take(len - 1);
            let first_part = first_part.collect::<Vec<_>>().join(", ");

            let last = iter.last().unwrap().to_string();
            format!("{}, {} {}", first_part, conjunction, last)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_mod() {
        assert_eq!(prefix_mod("hello", "world", true), "worldhello");
        assert_eq!(prefix_mod("worldhello", "world", false), "hello");
    }

    #[test]
    fn test_join_conjunction_abc() {
        let iter = ["a", "b", "c"].into_iter();
        let joined = join_words(iter, "and", None);
        assert_eq!(joined, Some("a, b, and c".to_owned()));
    }

    #[test]
    fn test_join_conjunction_ab() {
        let iter = ["a", "b"].into_iter();
        let joined = join_words(iter, "or", None);
        assert_eq!(joined, Some("a or b".to_owned()));
    }

    #[test]
    fn test_join_conjunction_a() {
        let joined = join_words(["a"].into_iter(), "or", None);
        assert_eq!(joined, Some("a".to_owned()));

        let joined = join_words(["a"].into_iter(), "or", Some("Only"));
        assert_eq!(joined, Some("Only a".to_owned()));
    }

    #[test]
    fn test_join_conjunction_empty() {
        let iter = std::iter::empty::<&str>();
        assert_eq!(join_words(iter.clone(), "or", None), None);
        assert_eq!(join_words(iter.clone(), "or", Some("Only")), None);
    }
}
