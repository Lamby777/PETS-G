/// Trait for user-facing descriptions of values.
pub trait Describe {
    /// Return a translated string describing the value.
    fn describe(&self) -> godot::builtin::GString;
}
