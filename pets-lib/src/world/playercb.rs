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

    pub fn party_pchars(&self) -> Vec<PChar> {
        self.party.iter().map(|v| v.bind().pchar).collect()
    }

    pub fn party_chardata(&self) -> Vec<CharData> {
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
        }

        self.past_positions.push(pos);
        self.past_rotations.push(rot.unwrap_or(self.last_rot()));

        self.move_chars(false);
        self.base_mut().set_global_position(pos);
    }

    /// Do all the movement calculations that need to run every tick.
    ///
    /// Returns whether the player is moving or not.
    fn calc_movements(&mut self, delta: f64) -> bool {
        let input = Input::singleton();
        let input_vector = normalized!(input.get_vector(
            "left".into(),
            "right".into(),
            "up".into(),
            "down".into(),
        ));
        let sprinting = input.is_action_pressed("sprint".into());
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

    pub fn good_guys_battlers(&self) -> Vec<Box<dyn Battler>> {
        self.party_chardata()
            .into_iter()
            .take(BATTLE_PARTY_SIZE)
            .map(|v| Box::new(v) as Box<dyn Battler>)
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
            _bad_guys: self.bad_guys_battlers(),
        }
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerCB {
    fn ready(&mut self) {
        self.party = load_pchar_scenes_under!(
            self;
            PChar::ETHAN,
            PChar::NEOXYLIN,
            PChar::SIVA,
            PChar::TERRA,
            PChar::MIRA,
            PChar::DYLAN,
            PChar::LEO,
            PChar::LYEMBO,
            PChar::QUOLO,
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
