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

    /// The enemies that are currently in battle with you
    pub battling: Vec<String>,

    pub tpbeacon_debounce: bool,
}

#[godot_api]
impl PlayerCB {
    pub fn singleton() -> Gd<Self> {
        current_scene().get_node_as("%PlayerCB")
    }

    pub fn party_ids(&self) -> Vec<String> {
        self.party
            .iter()
            .map(|v| v.get_name().to_string())
            .collect()
    }

    pub fn party_chardata(&self) -> Vec<CharData> {
        let si = StatsInterface::singleton();
        let si = si.bind();

        self.party_ids()
            .into_iter()
            .map(|id| si.get_character(&id))
            .collect()
    }

    /// Get the fx rectangle that follows the player
    pub fn fx_rect() -> Gd<ColorRect> {
        Self::singleton().get_node_as("BattleIntroRect")
    }

    /// Get the shader material of the fx rect
    pub fn fx_material() -> Gd<ShaderMaterial> {
        Self::fx_rect().get_material().unwrap().cast()
    }

    /// Checks all the possible things that could prevent movement.
    ///
    /// This includes things like:
    /// * Cutscenes
    /// * Menus
    pub fn can_move(&self) -> bool {
        // PRAISE SHORT-CIRCUIT EVALUATION!!
        let cant_move = DialogBox::singleton().bind().is_active()
            || self.is_in_battle()
            || self.tpbeacon_debounce;

        !cant_move
    }

    pub fn is_in_battle(&self) -> bool {
        !self.battling.is_empty()
    }

    /// Set character positions based on past pos/rot
    pub fn move_chars(&mut self, moving: bool) {
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

    pub fn teleport(
        &mut self,
        pos: Vector2,
        rot: Option<Vector2>,
        clear_past: bool,
    ) {
        if clear_past {
            self.past_positions.clear();
            self.past_rotations.clear();
        } else {
            self.past_positions.push(pos);

            let rot = rot.unwrap_or(self.last_rot());
            self.past_rotations.push(rot);
        }

        self.move_chars(false);
        self.base_mut().set_global_position(pos);
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
                self.last_rot()
            })
        }

        moving
    }

    fn last_rot(&self) -> Vector2 {
        self.past_rotations.get(0).cloned().unwrap_or(Vector2::ZERO)
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCB {
    fn ready(&mut self) {
        self.party = load_pchar_scenes_under!(
            self;
            PChar::ETHAN,
            PChar::SIVA,
            PChar::TERRA,
            PChar::MIRA,
            PChar::DYLAN,
            PChar::LEO,
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
