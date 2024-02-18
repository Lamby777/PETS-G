//!
//! General-purpose Godot/Rust helper stuff
//!

use godot::engine::tween::TransitionType;
use godot::engine::{Engine, RichTextLabel, Theme, Tween};
use godot::prelude::*;

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
        godot_tree().change_scene_to_file(concat!("res://scenes/", $scene, ".tscn").into())
    };
}

/// List elements separated by commas, but put a word
/// like "or" or "and" before the last one to sound less robotic.
///
/// # Edge Cases
/// - no commas if there are only two elements
/// - nothing happens if there's only one element
/// - returns None if the list is empty
///
/// Song:    "Conjunction Junction, what's your function?"
/// P/E/T/S: "This one right here, of course!"
fn join_words<T: ToString>(list: &[T], conjunction: &str) -> Option<String> {
    Some(match list.len() {
        0 => return None,
        1 => list[0].to_string(),
        2 => {
            format!(
                "{} {} {}",
                list[0].to_string(),
                conjunction,
                list[1].to_string()
            )
        }

        _ => {
            let iter = list.iter().map(|x| x.to_string());
            let first_part = iter.take(list.len() - 1);
            let first_part = first_part.collect::<Vec<_>>().join(", ");

            let last = list.last().unwrap().to_string();
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
        assert_eq!(
            join_words(&["a", "b", "c"], "and"),
            Some("a, b, and c".to_owned())
        );
    }

    #[test]
    fn test_join_conjunction_ab() {
        assert_eq!(join_words(&["a", "b"], "or"), Some("a or b".to_owned()));
    }

    #[test]
    fn test_join_conjunction_a() {
        assert_eq!(join_words(&["a"], "or"), Some("a".to_owned()));
    }

    #[test]
    fn test_join_conjunction_empty() {
        assert_eq!(join_words::<&str>(&[], "or"), None);
    }
}
