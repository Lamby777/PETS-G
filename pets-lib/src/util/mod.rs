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

use crate::functions::ScriptExecutor;
use crate::prelude::*;

use godot::engine::{Engine, Expression, SceneTreeTimer};
use godot::prelude::*;

// this is a macro so we can easily expand it and delete the definition
// when `gdext` adds new methods for allowing zero vectors
pub use crate::normalized;
#[macro_export]
macro_rules! normalized {
    ($vector:expr) => {{
        if $vector == Vector2::ZERO {
            Vector2::ZERO
        } else {
            Vector2::normalized($vector)
        }
    }};
}

/// Evaluate a GDScript string.
/// They are all evaluated from the context of the PlayerCB.
pub fn eval(script: &str) -> GReturn {
    let mut expr = Expression::new_gd();
    expr.parse(script.into());

    // nice try, buddy. i'm NOT calling it that.
    let executor = ScriptExecutor::singleton();

    // let res = expr.execute_ex().base_instance(executor.upcast()).done();
    let res = expr.call("execute".into(), &[
        varray![].to_variant(),
        executor.to_variant(),
    ]);

    Ok(res)
}

pub fn replace_str_all(text: &str, replaces: &[(String, String)]) -> String {
    replaces
        .into_iter()
        .fold(text.to_owned(), |text, (from, to)| text.replace(from, to))
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

pub fn start_ix(name: impl Into<String>) {
    DialogBox::singleton().bind_mut().start_ix(name.into());
}

pub fn start_ix_replace<S>(name: impl Into<String>, replace: &[(S, S)])
where
    S: Into<String> + Clone,
{
    let replace = replace
        .iter()
        .map(|(a, b)| (a.clone().into(), b.clone().into()))
        .collect::<Vec<_>>();

    DialogBox::singleton()
        .bind_mut()
        .start_ix_replace(name.into(), replace.to_vec());
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
