use godot::engine::{AnimatedSprite2D, IStaticBody2D, StaticBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct WalkingEnemy {
    base: Base<StaticBody2D>,
    sprite: OnReady<Gd<AnimatedSprite2D>>,
}

#[godot_api]
impl WalkingEnemy {
    #[func]
    pub fn anim_move(&mut self, moving: bool, backwards: bool) {
        let mode_str = if moving { "Run" } else { "Idle" };
        let dir_str = if backwards { "Back" } else { "" };

        // format!("{mode_str}{dir_str}");
    }
}

#[godot_api]
impl IStaticBody2D for WalkingEnemy {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            sprite: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let sprite = self.base().get_node_as("AnimatedSprite2D");
        self.sprite.init(sprite);
    }

    fn physics_process(&mut self, _delta: f64) {
        // walk towards player
        todo!()
    }
}
