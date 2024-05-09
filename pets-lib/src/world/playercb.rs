use godot::engine::{
    CharacterBody2D, ColorRect, ICharacterBody2D, ShaderMaterial,
};
use godot::prelude::*;

use crate::consts::playercb::*;
use crate::load_pchar_scenes_under;
use crate::prelude::*;

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
        current_scene().get_node_as("%PlayerCB")
    }

    pub fn fx_rect() -> Gd<ColorRect> {
        Self::singleton().get_node_as("BattleIntroRect")
    }

    pub fn fx_material() -> Gd<ShaderMaterial> {
        Self::fx_rect().get_material().unwrap().cast()
    }

    /// Checks all the possible things that could prevent movement.
    ///
    /// This includes things like:
    /// * Cutscenes
    /// * Menus
    pub fn can_move(&self) -> bool {
        let di = DBoxInterface::singleton();
        let dbox_active = di.bind().has_active_dbox();

        !dbox_active
    }

    /// Set character positions based on past pos/rot
    fn move_chars(&mut self, moving: bool) {
        if self.past_positions.len() == 0 {
            return;
        }

        for (i, ch) in self.party.iter_mut().enumerate() {
            // index of past data limqs
            let nth = i * PERSONAL_SPACE;
            ch.set_global_position(*self.past_positions.get_or_last(nth));

            let mut ch = ch.bind_mut();
            ch.anim_move(moving, *self.past_rotations.get_or_last(nth));
        }
    }

    /// Do all the movement calculations that need to run every tick.
    ///
    /// Returns whether the player is moving or not.
    fn calc_movements(&mut self, delta: f64) -> bool {
        let input = Input::singleton();
        let input_vector = input
            .get_vector(
                "left".into(),
                "right".into(),
                "up".into(),
                "down".into(),
            )
            .normalized();
        let sprinting = input.is_action_pressed("sprint".into());
        let moving = input_vector != Vector2::ZERO;

        let target_pos = if moving {
            let spr = if sprinting { SPRINT_COEFFICIENT } else { 1.0 };
            input_vector * MAX_SPEED * spr
        } else {
            Vector2::ZERO
        };

        let mut deltatimes = if moving { ACCELERATION } else { FRICTION };
        deltatimes *= delta as f32;

        let velocity = self.base().get_velocity();
        self.base_mut()
            .set_velocity(velocity.move_toward(target_pos, deltatimes as f32));

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

        moving
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
        let mut moving = false;

        if self.can_move() {
            moving = self.calc_movements(delta);
        }

        self.move_chars(moving);
    }
}
