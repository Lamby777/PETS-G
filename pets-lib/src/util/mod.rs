//!
//! General-purpose Godot/Rust helper stuff
//!

pub mod choices;
pub mod limiq;
pub mod singleton;

use crate::{Autoload as _, DialogBox, PlayerCB, StatsInterface};

use derived_deref::{Deref, DerefMut};
use godot::engine::object::ConnectFlags;
use godot::engine::tween::TransitionType;
use godot::engine::{
    ColorRect, Engine, RichTextLabel, SceneTreeTimer, ShaderMaterial, Theme,
    Tween,
};
use godot::prelude::*;

pub fn disconnect_signal<N, SN>(node: &mut Gd<N>, signal: SN)
where
    N: Inherits<Node>,
    SN: Into<StringName> + Copy,
{
    let node = node.upcast_mut::<Node>();

    node.get_signal_connection_list(signal.into())
        .iter_shared()
        .for_each(|dict| {
            // let signal = dict.get("signal").unwrap();
            let callable = dict.get("callable").unwrap();

            node.disconnect(signal.into(), callable.to());
        })
}

pub fn tr(text: impl Into<StringName>) -> GString {
    Engine::singleton().tr(text.into())
}

/// Convenience function to fade opacity shaders on/off
pub fn fade_black<N>(black: Gd<N>, visible: bool, tween_time: f64)
where
    N: GodotClass + Inherits<ColorRect>,
{
    fade_black_f64(black, visible as u8 as f64, tween_time)
}

pub fn fade_black_f64<N>(black: Gd<N>, visible: f64, tween_time: f64)
where
    N: GodotClass + Inherits<ColorRect>,
{
    let material = black
        .upcast_ref()
        .get_material()
        .unwrap()
        .cast::<ShaderMaterial>();
    let material_id = material.instance_id();

    let callable = Callable::from_fn("set_shader_value", move |args| {
        let mut material = Gd::<ShaderMaterial>::from_instance_id(material_id);
        material.set_shader_parameter("opacity".into(), args[0].clone());

        Ok(Variant::nil())
    });

    let start_value = material.get_shader_parameter("opacity".into());

    tween_method(
        callable,
        start_value,
        visible.to_variant(),
        tween_time,
        TransitionType::QUAD,
    )
    .unwrap();
}

pub use crate::tr_replace;
/// Macro to call tr! and format the result.
///
/// Usage:
/// ```
/// tr_replace! {
///     "TRANSLATION_KEY";
///     format_key,
///     // ... repeat as many as you like
/// }
/// ```
#[macro_export]
macro_rules! tr_replace {
    ($tr_key:expr; $($key:ident),* $(,)?) => {{
        let template = $crate::util::tr($tr_key).to_string();
        $(
        let key = concat!("{", stringify!($key), "}");
        let val = &$key.to_string();
        let template = template.replace(key, val);
        )*

        template
    }};
}

pub use crate::connect;
/// Macro to connect stuff without using the annoying 2-line
/// `let callable = xxxxx` syntax.
///
/// Usage:
/// ```
/// connect! {
///     node_to_connect_to,       "signal_name",
///     node_containing_callable, "callable_name";
///     // ... repeat as many as you like
/// }
/// ```
#[macro_export]
macro_rules! connect {
    ($($con_node:expr,$signal:expr=>$cal_node:expr,$cal_name:expr);* $(;)?) => {
        $({
            let callable = $cal_node.callable($cal_name);
            $con_node.connect($signal.into(), callable);
        })*
    };
}

/// Returns the singleton instance of `PlayerCB`.
/// So common that I might as well abbreviate it. :P
pub fn pcb() -> Gd<PlayerCB> {
    PlayerCB::try_singleton().unwrap()
}

/// Returns the singleton instance `StatsInterface`.
/// So common that I might as well abbreviate it. :P
pub fn si() -> Gd<StatsInterface> {
    StatsInterface::singleton()
}

#[derive(Deref, DerefMut)]
/// Wrapper around Gd<T> so I can implement external traits on godot stuff
pub struct GdW<T: GodotClass>(pub Gd<T>);

pub fn start_ix(name: impl Into<String>) {
    DialogBox::try_singleton()
        .unwrap()
        .bind_mut()
        .start_ix(name.into());
}

/// Find n where the nth child of type `Filter` is named `name`.
pub fn _index_of_child_with_name<Filter, N>(
    node: Gd<N>,
    name: GString,
) -> Option<usize>
where
    Filter: Inherits<Node>,
    N: Inherits<Node>,
{
    subchildren_of_type::<Filter, _>(node.upcast())
        .iter()
        .position(|n| n.upcast_ref().get_name() == name.clone().into())
}

pub fn connect_deferred<T>(node: &mut Gd<T>, signal: &str, callable: Callable)
where
    T: Inherits<Object>,
{
    node.upcast_mut()
        .connect_ex(signal.into(), callable)
        .flags(ConnectFlags::DEFERRED.ord() as u32)
        .done();
}

/// Like setTimeout in JS, using godot timers.
/// Uses SECONDS, not ms.
pub fn set_timeout<F>(time_sec: f64, mut func: F) -> Gd<SceneTreeTimer>
where
    F: FnMut() + Sync + Send + 'static,
{
    let callable = Callable::from_fn("timeout", move |_| {
        func();
        Ok(Variant::nil())
    });

    set_timeout_callable(time_sec, callable)
}

/// Like `set_timeout`, but accepts a `Callable` instead of a closure.
pub fn set_timeout_callable(
    time_sec: f64,
    callable: Callable,
) -> Gd<SceneTreeTimer> {
    let mut timer = godot_tree().create_timer(time_sec).unwrap();
    timer.connect("timeout".into(), callable);

    timer
}

pub fn mark_input_handled<T>(node: &Gd<T>)
where
    T: Inherits<Node>,
{
    node.upcast_ref::<Node>()
        .get_viewport()
        .unwrap()
        .set_input_as_handled();
}

/// Recursively get all children of a node that are of a certain type.
/// Returns a vector. Use `subchildren_of_type_array` for a godot array.
///
/// bugfix later: it won't find children of nodes that are the correct type
pub fn subchildren_of_type<T, Par>(parent: Gd<Par>) -> Vec<Gd<T>>
where
    Par: Inherits<Node>,
    T: Inherits<Node>,
{
    let mut res = vec![];
    let parent = parent.upcast_ref();

    for node in parent.get_children().iter_shared() {
        let Ok(node) = node.clone().try_cast::<T>() else {
            let children = subchildren_of_type::<T, _>(node);
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

pub fn tween_method<V>(
    callable: Callable,
    start_value: V,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    V: ToGodot,
{
    let res: Option<_> = try {
        let mut tween = godot_tree().create_tween()?;

        tween
            .tween_method(
                callable,
                start_value.to_variant(),
                end_value.to_variant(),
                time,
            )?
            .set_trans(trans);

        tween
    };

    res.ok_or(())
}

/// shorthand to do some tweeneroonies :3
/// `time` is in seconds
pub fn tween<NP, V, N>(
    mut node: Gd<N>,
    property: NP,
    start_value: Option<V>,
    end_value: V,
    time: f64,
    trans: TransitionType,
) -> Result<Gd<Tween>, ()>
where
    NP: Into<NodePath>,
    V: ToGodot,
    N: Inherits<Node> + Inherits<Object>,
{
    let res: Option<_> = try {
        let mut tween = node.upcast_mut::<Node>().create_tween()?;

        let mut property = tween
            .tween_property(
                node.upcast::<Object>(),
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
