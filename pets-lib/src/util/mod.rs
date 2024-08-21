//!
//! General-purpose Godot/Rust helper stuff
//!

pub mod choices;
mod describe;
mod extensions;
pub mod limiq;
mod node_stuff;
pub mod registry;
pub mod singleton;

pub use describe::Describe;
pub use extensions::*;
pub use node_stuff::*;

use godot::classes::{Engine, SceneTreeTimer};
use godot::prelude::*;

pub fn month_string_3letter(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => panic!("Invalid month"),
    }
}

pub fn tr(text: impl Into<StringName>) -> GString {
    Engine::singleton().tr(text.into())
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
