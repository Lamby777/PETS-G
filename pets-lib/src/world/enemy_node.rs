use godot::engine::{AnimatedSprite2D, IStaticBody2D, StaticBody2D};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=StaticBody2D)]
pub struct WalkingEnemy {
    base: Base<StaticBody2D>,

    // #[export]
    // enemy_id: EnemyID,
    #[init(default = onready_node(&base, "AnimatedSprite2D"))]
    sprite: OnReady<Gd<AnimatedSprite2D>>,
}

#[godot_api]
impl WalkingEnemy {
    /// Set the animation of the enemy
    ///
    /// # Arguments
    ///
    /// * `moving` - If true, plays running anims, else idle.
    /// * `flipped` - If true, flips the sprite horizontally.
    /// * `backwards` - If true, play backwards-facing sprites.
    ///
    /// The running sprites are drawn by default facing left.
    /// `flipped` should be true if the enemy is facing right.
    ///
    /// Backwards should be used if the enemy is below the player
    /// in terms of Y position, so they'd be running up the screen.
    #[func]
    pub fn anim_move(&mut self, moving: bool, flipped: bool, _backwards: bool) {
        let mode_str = if moving { "Run" } else { "Idle" };

        // TODO uncomment when backwards sprites are added
        // let dir_str = if backwards { "Back" } else { "" };
        // let anim_name = format!("{mode_str}{dir_str}");

        self.sprite.set_animation(mode_str.into());
        self.sprite.set_flip_h(flipped);
    }
}

#[godot_api]
impl IStaticBody2D for WalkingEnemy {
    fn physics_process(&mut self, _delta: f64) {
        // walk towards player
    }
}
