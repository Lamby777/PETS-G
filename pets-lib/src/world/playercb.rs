use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::consts::playercb::*;
use crate::{load_pchar_scenes_under, prelude::*};

use super::pchar_node::PCharNode;

/// This scene contains the "player" aka the invisible
/// entity that is moved around with WASD. It also contains
/// party members as scenes, and this script does stuff like
/// running animations on those nodes too.
#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct PlayerCB {
    base: Base<CharacterBody2D>,
    party: Vec<Gd<PCharNode>>,

    #[init(default = LimiQ::new(2000))]
    past_positions: LimiQ<Vector2>,

    #[init(default = LimiQ::new(2000))]
    past_rotations: LimiQ<Vector2>,
}

#[godot_api]
impl PlayerCB {
    pub fn singleton() -> Gd<Self> {
        godot_tree()
            .get_first_node_in_group("playercb".into())
            .expect("PlayerCB group had no nodes...?")
            .cast()
    }

    fn move_chars(&mut self, moving: bool) {
        if self.past_positions.len() == 0 {
            return;
        }

        for (i, ch) in self.party.iter_mut().enumerate() {
            // index of past data limqs
            let nth = i * PERSONAL_SPACE as usize;

            ch.set_global_position(*self.past_positions.get_or_last(nth));

            {
                let mut ch = ch.bind_mut();
                ch.anim_move(moving, *self.past_rotations.get_or_last(nth));
            }
        }
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCB {
    fn ready(&mut self) {
        self.party = load_pchar_scenes_under!(
            self;
            "agent_e",
            "agent_s",
            "agent_t",
            "mira",
            "dubs",
            "yoyo",
        );
    }

    fn physics_process(&mut self, delta: f64) {
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

        let velocity = self.base().get_velocity();
        self.base_mut()
            .set_velocity(velocity.move_toward(toward, deltatimes as f32));

        self.base_mut().move_and_slide();

        let pos_updated = (self.past_positions.len() == 0)
            || (self.past_positions[0] != self.base().get_position());

        if pos_updated {
            self.past_positions.push(self.base().get_global_position());

            // don't push new input vector if slowing down
            self.past_rotations.push(if moving {
                input_vector
            } else {
                self.past_rotations.get(0).cloned().unwrap_or(Vector2::ZERO)
            })
        }

        self.move_chars(moving)
    }
}
