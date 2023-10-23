use godot::engine::CharacterBody2D;
use godot::prelude::*;

use crate::prelude::*;

// Movement physics stuff
const ACCELERATION: f64 = 3000.0;
const FRICTION: f64 = 2500.0;
const MAX_SPEED: f64 = 320.0;

// Distance between party members
const PERSONAL_SPACE: u16 = 15;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PlayerCB {
    #[base]
    node: Base<CharacterBody2D>,
    si: Gd<StatsInterface>,
    velocity: Vector2,
}

#[godot_api]
impl PlayerCB {
    fn _physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let input_vector = input
            .get_vector("left".into(), "right".into(), "up".into(), "down".into())
            .normalized();
        let moving = input_vector != Vector2::ZERO;

        let (toward, deltatimes) = if moving {
            (
                input_vector * real::from_f64(MAX_SPEED),
                delta * ACCELERATION,
            )
        } else {
            (Vector2::ZERO, (delta * FRICTION))
        };

        self.velocity = self.velocity.move_toward(toward, deltatimes as f32);
    }

    //
    // move_and_slide()
    //
    // var posUpdated: bool = (
    //   (pastPositions.get_len() == 0) or
    //   (pastPositions.get_at(0) != position)
    // )
    //
    // if posUpdated:
    //   pastPositions.push_front(global_position)
    //   # don't push new input vector if slowing down
    //   pastRotations.push_front(
    //     input_vector
    //     if moving else
    //     pastRotations.get_first_or(Vector2(0, 0))
    //   )
    //
    // move_chars(moving)
}

/*
"""
"""

# Movement physics stuff
const ACCELERATION  := 3000
const FRICTION    := 2500
const MAX_SPEED    := 320

# Distance between party members
const PERSONAL_SPACE := 15

@onready var agentE = $AgentE
@onready var agentS = $AgentS
@onready var agentT = $AgentT

var current_music_zone: Polygon2D

var pastPositions := LimitedQueue.new(2000)
var pastRotations := LimitedQueue.new(2000)
@onready var party: Array[PChar] = [
  agentE,
  agentS,
  agentT,
]

func move_chars(moving: bool):
  if pastPositions.get_len() == 0: return

  for i in party.size():
    var ch := party[i]

    # index of past data limqs
    var nth = i * PERSONAL_SPACE

    ch.global_position = pastPositions.get_or_last(nth)
    ch.anim_move(moving, pastRotations.get_or_last(nth))

            */
