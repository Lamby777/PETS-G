use godot::engine::{Node2D, Node2DVirtual, Sprite2D};
use godot::prelude::*;

const SPEED: f32 = 200.0;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for Player {
    fn init(node: Base<Node2D>) -> Self {
        Self { node }
    }

    fn process(&mut self, delta: f64) {
        // let mut ply = self.node.get_node_as::<Sprite2D>("Sprite2D");

        let input = Input::singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if input.is_action_pressed("battle_move_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("battle_move_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("battle_move_down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("battle_move_up".into()) {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * SPEED;
        }

        let change = velocity * real::from_f64(delta);
        let position = self.node.get_global_position() + change;
        let position = Vector2::new(position.x.clamp(0.0, 1920.0), position.y.clamp(0.0, 1080.0));
        self.node.set_global_position(position);
    }
}
