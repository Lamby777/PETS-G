use godot::classes::AnimationPlayer;
use godot::prelude::*;

pub trait AnimationPlayerExt {
    fn play_animation_forwards(
        &mut self,
        anim: impl Into<GString>,
        forward: bool,
    );
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
    fn play_animation_forwards(
        &mut self,
        anim: impl Into<GString>,
        forward: bool,
    ) {
        self.set_assigned_animation(anim.into());
        self.play_forwards(forward);
    }
}
