use godot::engine::{
    AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D,
};
use godot::prelude::*;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct AnimOptions {
    pub moving: bool,
    pub flipped: Option<bool>,
    pub backwards: bool,
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct WalkingEnemy {
    base: Base<CharacterBody2D>,

    #[export]
    enemy_id: GString,

    #[export]
    sight_range: real,

    #[export]
    max_speed: real,

    #[init(default = onready_node(&base, "AnimatedSprite2D"))]
    sprite: OnReady<Gd<AnimatedSprite2D>>,

    #[init(default = onready_node(&base, "ContactRange"))]
    range: OnReady<Gd<Area2D>>,

    // returns early from `anim_move` if the same options are passed
    anim_debounce: Option<AnimOptions>,

    ready: bool,
    touched_player: bool,
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
    fn anim_move(&mut self, opts: AnimOptions) {
        // only run if the options have changed
        if Some(opts) == self.anim_debounce {
            return;
        }

        self.anim_debounce = Some(opts);

        let mode_str = if opts.moving { "Run" } else { "Idle" };

        // TODO uncomment when backwards sprites are added
        let dir_str = ""; // if opts.backwards { "Back" } else { "" };

        let anim_name = format!("{}-{}{}", self.enemy_id, mode_str, dir_str);
        self.sprite.set_animation(anim_name.into());
        self.sprite.play();

        if let Some(v) = opts.flipped {
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

    /// Returns the positions of the enemy and the player
    fn self_pos_pcb_pos(&self) -> ((real, real), (real, real)) {
        let pcb = PlayerCB::singleton();
        let pcb_pos = pcb.get_global_position();
        let self_pos = self.base().get_global_position();
        (self_pos.to_tuple(), pcb_pos.to_tuple())
    }

    pub fn walk_towards_player(&mut self, _delta: f64) {
        let spd = self.max_speed;
        {
            let mut base = self.base_mut();

            let pcb_pos = PlayerCB::singleton().get_position();
            let own_pos = base.get_position();

            let target_pos = (pcb_pos - own_pos).normalized();
            if own_pos.distance_to(pcb_pos) < 10.0 {
                return;
            }

            base.set_velocity(target_pos * spd);
            base.move_and_slide();

            base.look_at(pcb_pos);
        }

        self.sprite.set_global_rotation(0.0);
    }

    #[func]
    fn on_player_touched(&mut self, _body: Gd<Node2D>) {
        if self.touched_player {
            return;
        }

        self.touched_player = true;

        godot_print!("Player touched enemy: {}", self.enemy_id);
        World::start_battle(self.enemy_id.clone());
    }
}

#[godot_api]
impl ICharacterBody2D for WalkingEnemy {
    fn ready(&mut self) {
        // check to make sure it's a valid enemy id
        let enemy_id = self.enemy_id.to_string();
        if !EnemyID::ALL.contains(&enemy_id.as_str()) {
            panic!("Invalid enemy id: {}", enemy_id);
        }

        let callable = self.base().callable("on_player_touched");
        self.range.connect("body_entered".into(), callable);

        self.ready = true;
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.ready {
            return;
        }

        if !self.is_player_in_sight() {
            // if far from player, play idle and face forward
            self.anim_move(AnimOptions {
                moving: false,
                flipped: None,
                backwards: false,
            });
        } else {
            let ((own_x, own_y), (pcb_x, pcb_y)) = self.self_pos_pcb_pos();

            // flipped if player is to the right of the enemy
            let flipped = pcb_x > own_x;
            let backwards = pcb_y < own_y;

            self.anim_move(AnimOptions {
                moving: true,
                flipped: Some(flipped),
                backwards,
            });

            // if close enough to player, run at them
            self.walk_towards_player(delta);
        }
    }
}
