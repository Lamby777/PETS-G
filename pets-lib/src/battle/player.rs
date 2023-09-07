//!
//! Player icon that moves around n shit during battles
//!

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

const SPEED: f32 = 600.0;

const DIRECTIONS: [(&'static str, Vector2); 4] = [
    ("up", Vector2::UP),
    ("down", Vector2::DOWN),
    ("left", Vector2::LEFT),
    ("right", Vector2::RIGHT),
];

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleIcon {
    #[base]
    node: Base<Node2D>,

    velocity: Vector2,
}

#[godot_api]
impl Node2DVirtual for BattleIcon {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,

            velocity: Vector2::new(0.0, 0.0),
        }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        for (dir_s, dir_vec2) in DIRECTIONS {
            let input_mapping_name = format!("battle_move_{dir_s}",);

            if input.is_action_pressed(input_mapping_name.into()) {
                self.velocity += dir_vec2;
            }
        }

        if self.velocity.length() > 0.0 {
            self.velocity = self.velocity.normalized() * SPEED;
        }

        let change = self.velocity * real::from_f64(delta);

        let position = self.node.get_global_position() + change;
        let position = Vector2::new(position.x.clamp(0.0, 1920.0), position.y.clamp(0.0, 1080.0));
        self.node.set_global_position(position);
    }
}
