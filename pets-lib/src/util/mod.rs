//!
//! General-purpose Godot/Rust helper stuff
//!

pub mod choices;
mod describe;
mod extensions;
pub mod limiq;
mod node_stuff;
pub mod singleton;

pub use describe::Describe;
pub use extensions::*;
pub use node_stuff::*;

use godot::classes::{Engine, SceneTreeTimer};
use godot::meta::AsArg;
use godot::prelude::*;

/// Parameter `month` is the month number (1 = Jan, 2 = Feb, etc.)
pub fn shortened_month_string(month: u32) -> GString {
    match month {
        1..=12 => tr(&format!("MONTH_SHORTENED_{}", month)),
        _ => panic!("invalid month, outside of 1-12 range: {}", month),
    }
}

pub fn tr(text: impl AsArg<StringName>) -> GString {
    Engine::singleton().tr(text)
}

pub use crate::tr_replace;
/// Macro to call tr! and format the **result**.
///
/// To format the input, just use `tr()` with `format!()`. That's not what this is for.
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
    let callable = Callable::from_local_fn("timeout", move |_| {
        func();
        Ok(Variant::nil())
    });

    set_timeout_callable(time_sec, callable)
}

/// Like [set_timeout], but accepts a [Callable] instead of a closure.
pub fn set_timeout_callable(
    time_sec: f64,
    callable: Callable,
) -> Gd<SceneTreeTimer> {
    let mut timer = godot_tree().create_timer(time_sec).unwrap();
    timer.connect("timeout", &callable);

    timer
}
