use godot::builtin::GString;

/// Trait for user-facing descriptions of values.
pub trait Describe {
    /// Return a translated string describing the value.
    fn describe(&self) -> GString;
}
