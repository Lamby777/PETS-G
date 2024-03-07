use godot::engine::{AnimatedSprite2D, IStaticBody2D, StaticBody2D};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=StaticBody2D)]
pub struct WalkingEnemy {
    base: Base<StaticBody2D>,

    #[export]
    enemy_id: GString,

    #[export]
    sight_range: real,

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
    pub fn anim_move(&mut self, moving: bool, flipped: Option<bool>, _backwards: bool) {
        let mode_str = if moving { "Run" } else { "Idle" };

        // TODO uncomment when backwards sprites are added
        let dir_str = ""; // if backwards { "Back" } else { "" };

        let anim_name = format!("{}-{}{}", self.enemy_id, mode_str, dir_str);

        self.sprite.set_animation(anim_name.into());

        if let Some(v) = flipped {
            self.sprite.set_flip_h(v);
        }
    }

    pub fn distance_to_player(&self) -> real {
        let pcb = PlayerCB::singleton();
        let pcb_pos = pcb.get_global_position();
        let self_pos = self.base().get_global_position();

        self_pos.distance_to(pcb_pos)
    }

    pub fn is_player_in_sight(&self) -> bool {
        self.distance_to_player() < self.sight_range
    }

    pub fn walk_towards_player(&mut self, _delta: f64) {}
}

#[godot_api]
impl IStaticBody2D for WalkingEnemy {
    fn ready(&mut self) {
        // check to make sure it's a valid enemy id
        let enemy_id = self.enemy_id.to_string();
        if !EnemyID::ALL.contains(&enemy_id.as_str()) {
            panic!("Invalid enemy id: {}", enemy_id);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.is_player_in_sight() {
            // if far from player, play idle and face forward
            self.anim_move(false, None, false);
        } else {
            let pcb = PlayerCB::singleton();
            let Vector2 { x: pcb_x, y: pcb_y } = pcb.get_global_position();
            let Vector2 { x: own_x, y: own_y } = self.base().get_global_position();

            // flipped if player is to the right of the enemy
            let flipped = pcb_x > own_x;
            let backwards = pcb_y < own_y;

            self.anim_move(true, Some(flipped), backwards);

            // if close enough to player, run at them
            self.walk_towards_player(delta);
        }
    }
}
