use std::cmp::Ordering;

use godot::engine::{
    CharacterBody2D, ColorRect, ICharacterBody2D, ShaderMaterial,
};
use godot::prelude::*;

use crate::consts::playercb::*;
use crate::load_pchar_scenes_under;
use crate::prelude::*;

use super::inv_node::InventoryNode;
use super::pchar_node::PCharNode;
use super::BATTLE_PARTY_SIZE;

/// The player will stop being controlled once it reaches this
/// distance from the cutscene target.
const CUTSCENE_MOTION_CLOSE_ENOUGH: f32 = 10.0;

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
    pub battling: Vec<Rc<RefCell<EnemyData>>>,

    pub tpbeacon_debounce: bool,
    pub in_water: bool,

    /// if the player is currently being controlled by a script,
    /// this is the location the player is being moved to
    pub cutscene_motion: Option<Vector2>,

    #[init(default = 1.0)]
    pub water_speed_mod: real,
}

#[godot_api]
impl PlayerCB {
    #[signal]
    fn teleported(&self, target: Gd<Node2D>);

    #[func]
    pub fn singleton() -> Gd<Self> {
        World::singleton().get_node_as("%PlayerCB")
    }

    #[signal]
    fn motion_done(&self);

    #[func]
    pub fn move_to_absolute(&mut self, x: real, y: real) {
        let end = Vector2::new(x, y);
        self.cutscene_motion = Some(end);
    }

    #[func]
    pub fn move_to_relative(&mut self, x: real, y: real) {
        let end = Vector2::new(x, y);
        let start = self.base().get_global_position();
        let total = start + end;
        self.cutscene_motion = Some(total);
    }

    pub fn party_pchars(&self) -> Vec<PChar> {
        self.party.iter().map(|v| v.bind().pchar).collect()
    }

    pub fn party_chardata(&self) -> Vec<Rc<RefCell<CharData>>> {
        self.party_pchars()
            .into_iter()
            .map(|id| si().bind().get_character(&id))
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
        let dbox_is_active = DialogBox::singleton().bind().is_active();

        let cant_move = dbox_is_active
            || InventoryNode::singleton().bind().is_open()
            || self.is_in_battle()
            || self.tpbeacon_debounce
            || self.cutscene_motion.is_some();

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
        }

        self.past_positions.push(pos);
        self.past_rotations.push(rot.unwrap_or(self.last_rot()));

        self.move_chars(false);
        self.base_mut().set_global_position(pos);
    }

    /// Do all the movement calculations that need to run every tick.
    ///
    /// Returns whether the player is moving or not.
    fn calc_movements(
        &mut self,
        input_vector: Vector2,
        sprinting: bool,
        delta: f64,
    ) -> bool {
        let moving = input_vector != Vector2::ZERO;

        let target_pos = if moving {
            let spr = if sprinting { SPRINT_COEFFICIENT } else { 1.0 };
            input_vector * MAX_SPEED * spr * self.water_speed_mod
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

    pub fn good_guys_battlers(&self) -> Vec<Rc<RefCell<dyn Battler>>> {
        self.party_chardata()
            .into_iter()
            .take(BATTLE_PARTY_SIZE)
            .map(|v| v as Rc<RefCell<dyn Battler>>)
            .collect()
    }

    pub fn bad_guys_battlers(&self) -> Vec<Rc<RefCell<dyn Battler>>> {
        self.battling
            .iter()
            .cloned()
            .map(|v| v as Rc<RefCell<dyn Battler>>)
            .collect()
    }

    pub fn new_battlers(&self) -> Battlers {
        Battlers {
            good_guys: self.good_guys_battlers(),
            bad_guys: self.bad_guys_battlers(),
        }
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCB {
    fn ready(&mut self) {
        self.party = load_pchar_scenes_under!(
            self;
            PChar::MIRA,
            PChar::ETHAN,
            PChar::SIVA,
            PChar::TERRA,
        );
    }

    fn physics_process(&mut self, delta: f64) {
        let mut moving = false;

        if self.can_move() {
            let input = Input::singleton();
            let input_vector = normalized!(input.get_vector(
                "left".into(),
                "right".into(),
                "up".into(),
                "down".into(),
            ));
            let sprinting = input.is_action_pressed("sprint".into());

            moving = self.calc_movements(input_vector, sprinting, delta);
        } else if let Some(target) = self.cutscene_motion {
            // TODO separate function
            let own_pos = self.base().get_global_position();
            let cmp_x = target.x.partial_cmp(&own_pos.x).unwrap();
            let cmp_y = target.y.partial_cmp(&own_pos.y).unwrap();

            use Ordering::*;
            let iv_x = match cmp_x {
                Less => Vector2::LEFT,
                Greater => Vector2::RIGHT,
                Equal => Vector2::ZERO,
            };

            let iv_y = match cmp_y {
                Less => Vector2::UP,
                Greater => Vector2::DOWN,
                Equal => Vector2::ZERO,
            };

            let input_vector = iv_x + iv_y;
            let sprinting = false; // TODO
            moving = self.calc_movements(input_vector, sprinting, delta);

            if (target - own_pos).length() < CUTSCENE_MOTION_CLOSE_ENOUGH {
                self.cutscene_motion = None;
                self.base_mut().emit_signal("motion_done".into(), &[]);
                self.base_mut().set_global_position(target);
            }

            self.move_chars(moving);
            return;
        }

        self.move_chars(moving);
    }
}
