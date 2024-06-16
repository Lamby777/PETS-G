//!
//! Player icon that moves around n shit during battles
//!

use std::cell::LazyCell;

use godot::engine::Sprite2D;
use godot::prelude::*;

use super::stat_translation as to_battle;
use crate::prelude::*;

type DirectionalInputNames = [(StringName, Vector2); 4];

const BATTLE_DIRECTIONS: LazyCell<DirectionalInputNames> =
    LazyCell::new(|| {
        [
            ("battle_move_up".into(), Vector2::UP),
            ("battle_move_down".into(), Vector2::DOWN),
            ("battle_move_left".into(), Vector2::LEFT),
            ("battle_move_right".into(), Vector2::RIGHT),
        ]
    });

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BattleIcon {
    base: Base<Node2D>,

    /// Maximum speed of player icon
    #[init(default = 400.0)]
    speed: FloatStat,

    /// Acceleration amount per tick held
    #[init(default = 80.0)]
    acceleration: FloatStat,

    /// Coefficient of deceleration
    #[init(default = 0.96)]
    friction: FloatStat,

    /// Current velocity of player icon
    /// NOT normalized, but still limited by speed.
    #[init(default = Vector2::ZERO)]
    velocity: Vector2,
}

#[godot_api]
impl BattleIcon {
    fn pchar_to_frame(pchar: PChar) -> i32 {
        match pchar {
            PChar::ETHAN => 0,
            PChar::TERRA => 1,
            PChar::SIVA => 2,
            PChar::PORKY => 3,

            PChar::FUZZY => 8,

            _ => {
                godot_warn!("PChar {} doesn't have a battle icon (yet). Defaulting to Ethan's icon.", pchar);
                0
            }
        }
    }

    pub fn set_icon(&mut self, pchar: PChar) {
        let mut sprite = self.base().get_node_as::<Sprite2D>("Sprite2D");
        sprite.set_frame(Self::pchar_to_frame(pchar));
    }

    fn process_movement(&mut self, delta: f64) {
        let input = Input::singleton();

        self.velocity *= self.friction;

        // check inputs
        let mut input_vector = Vector2::ZERO;
        for (k, v) in BATTLE_DIRECTIONS.iter() {
            if input.is_action_pressed(k.clone()) {
                input_vector += *v;
            }
        }

        self.velocity += self.acceleration * input_vector;

        if self.velocity.length() > self.speed {
            self.velocity = normalized!(self.velocity) * self.speed;
        }

        let change = self.velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        self.base_mut().set_global_position(position);
    }
}

#[godot_api]
impl INode2D for BattleIcon {
    fn ready(&mut self) {
        let ch_speed = si().bind().natural_speed_of(PChar::ETHAN);
        self.speed = to_battle::speed(ch_speed);
    }

    fn process(&mut self, delta: f64) {
        self.process_movement(delta);
    }
}
