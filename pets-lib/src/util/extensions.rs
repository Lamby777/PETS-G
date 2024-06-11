use godot::builtin::Vector2;

pub trait Vector2Ext {
    fn to_tuple(&self) -> (f32, f32);
}

impl Vector2Ext for Vector2 {
    /// Convert the godot Vector2 into a tuple of x and y.
    fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
