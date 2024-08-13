//!
//! Dialog box class for menus and dialogue text
//!

use dialogical::prelude::*;
use godot::engine::global::Side;
use godot::engine::{
    AnimationPlayer, Control, HBoxContainer, IPanelContainer, InputEvent,
    PanelContainer, RichTextLabel, Timer,
};
use godot::prelude::*;

use crate::consts::dialogue::*;
use crate::prelude::*;

mod dchoice;
mod placeholders;
use dchoice::DChoice;

#[derive(Clone)]
pub struct MetaPair<T> {
    pub temporary: T,
    pub permanent: T,
}

impl<T> MetaPair<T> {
    pub fn from_cloned(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            temporary: v.clone(),
            permanent: v,
        }
    }

    /// matches over a `Metaline` to update a field depending on
    /// whether it's pageonly, permanent, or nochange
    pub fn set_from<'a>(&mut self, meta: &'a Metaline<T>)
    where
        T: Clone,
    {
        use Metaline::*;
        self.temporary = match meta {
            PageOnly(ref v) => v,
            Permanent(ref v) => {
                self.permanent = v.clone();
                v
            }
            NoChange => &self.permanent,
        }
        .clone();
    }
}
