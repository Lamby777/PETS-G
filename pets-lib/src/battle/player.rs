//!
//! Player icon that moves around n shit during battles
//!

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

use super::stat_translation::to_battle;
use crate::{prelude::*, show_dialog};

type DirectionalInputNames = [(&'static str, Vector2); 4];

const BATTLE_DIRECTIONS: DirectionalInputNames = [
    ("battle_move_up", Vector2::UP),
    ("battle_move_down", Vector2::DOWN),
    ("battle_move_left", Vector2::LEFT),
    ("battle_move_right", Vector2::RIGHT),
];

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleIcon {
    #[base]
    node: Base<Node2D>,
    si: Gd<StatsInterface>,

    /// Maximum speed of player icon
    speed: FloatStat,

    /// Acceleration amount per tick held
    acceleration: FloatStat,

    /// Coefficient of deceleration
    friction: FloatStat,

    /// Current velocity of player icon
    /// NOT normalized, but still limited by speed.
    velocity: Vector2,
}

#[godot_api]
impl BattleIcon {
    fn process_movement(&mut self, delta: f64) {
        let input = Input::singleton();

        self.velocity *= self.friction;

        // check inputs
        let mut input_vector = Vector2::new(0.0, 0.0);
        for (k, v) in BATTLE_DIRECTIONS.into_iter() {
            if input.is_action_pressed(k.into()) {
                input_vector += v;
            }
        }

        self.velocity += self.acceleration * input_vector;

        if self.velocity.length() > self.speed {
            self.velocity = self.velocity.normalized() * self.speed;
        }

        let change = self.velocity * real::from_f64(delta);
        let position = self.node.get_global_position() + change;
        self.node.set_global_position(position);
    }
}

#[godot_api]
impl Node2DVirtual for BattleIcon {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),

            speed: 400.0,
            acceleration: 80.0,
            friction: 0.96,
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    fn ready(&mut self) {
        let ch_speed = self.si.bind().natural_speed_of(PChar::ETHAN);
        self.speed = to_battle::speed(ch_speed);
    }

    fn process(&mut self, delta: f64) {
        self.process_movement(delta);
    }
}
