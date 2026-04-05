use godot::classes::AnimationPlayer;
use godot::meta::AsArg;
use godot::prelude::*;

pub trait AnimationPlayerExt {
    fn play_animation_forwards(&mut self, anim: &str, forward: bool);
    fn play_forwards(&mut self, forward: bool);
}

impl AnimationPlayerExt for AnimationPlayer {
    /// Play the selected animation from the start.
    /// Plays backwards if `forward` is false.
    fn play_forwards(&mut self, forward: bool) {
        match forward {
            true => self.play(),
            false => self.play_backwards(),
        }
    }

    /// Play the specified animation from the start.
    /// Plays backwards if `forward` is false.
    fn play_animation_forwards(&mut self, anim: &str, forward: bool) {
        self.set_assigned_animation(&StringName::from(anim));
        self.play_forwards(forward);
    }
}
