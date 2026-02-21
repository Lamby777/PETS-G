use std::cmp::Ordering;

use godot::classes::{
    CharacterBody2D, ColorRect, ICharacterBody2D, Input, ShaderMaterial,
};
use godot::prelude::*;

use crate::common::*;
use crate::consts::partycb::*;

use super::inv_node::InventoryNode;
use super::pchar_node::PCharNode;
// use super::BATTLE_PARTY_SIZE;

/// The player will stop being controlled once it reaches this
/// distance from the cutscene target.
pub const CUTSCENE_MOTION_CLOSE_ENOUGH: f32 = 1.0;
pub const CUTSCENE_MOTION_CLOSE_ENOUGH_SQUARED: f32 =
    CUTSCENE_MOTION_CLOSE_ENOUGH * CUTSCENE_MOTION_CLOSE_ENOUGH;

pub struct Inputs {
    pub input_vector: Vector2,
    pub sprinting: bool,
}

impl Inputs {
    /// Get the pair of -1, 0, or 1 required to get from one
    /// point to another.
    pub fn iv_from_to(from: Vector2, to: Vector2) -> Vector2 {
        let diff = to - from;
        let x = match diff.x.partial_cmp(&0.0).unwrap() {
            Ordering::Less => -1.0,
            Ordering::Equal => 0.0,
            Ordering::Greater => 1.0,
        };
        let y = match diff.y.partial_cmp(&0.0).unwrap() {
            Ordering::Less => -1.0,
            Ordering::Equal => 0.0,
            Ordering::Greater => 1.0,
        };

        // WARN: this vector is not normalized. this is intentional, but is technically "wrong."
        Vector2::new(x, y)
    }

    pub fn from_player_input() -> Self {
        let input = Input::singleton();

        // WARN: this vector is not normalized. this is intentional, but is technically "wrong."
        let input_vector = input.get_vector("left", "right", "up", "down");

        let sprinting = input.is_action_pressed("sprint");
        Inputs {
            input_vector,
            sprinting,
        }
    }
}

/// This scene contains the "player" aka the invisible entity that is
/// moved around with WASD. It also contains party members as scenes,
/// and this script does stuff like running animations on those nodes too.
#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct PartyCB {
    base: Base<CharacterBody2D>,

    /// Whether or this PCB will apply its movements to all party members' nodes
    /// using `past_positions` and `past_rotations`.
    #[export]
    #[init(val = false)]
    pub in_cutscene: bool,

    /// Each party member's scene node
    #[var]
    party: Array<Gd<PCharNode>>,

    #[init(val = LimiQ::new(2000))]
    past_positions: LimiQ<Vector2>,

    #[init(val = LimiQ::new(2000))]
    past_rotations: LimiQ<Vector2>,

    pub tpbeacon_debounce: bool,
    pub in_water: bool,

    #[init(val = 1.0)]
    pub water_speed_mod: real,

    /// pos1: target (global) position
    /// pos2: "old" (original) position before the motion
    pub cutscene_motion: Option<(Vector2, Vector2)>,
}

#[godot_api]
impl PartyCB {
    #[signal]
    fn teleported(target: Gd<Node2D>);

    #[signal]
    fn pcb_motion_done();

    #[func]
    pub fn singleton() -> Gd<Self> {
        World::singleton().get_node_as("%PartyCB")
    }

    #[func]
    pub fn move_to_absolute(&mut self, x: real, y: real) {
        let start = self.base().get_global_position();
        let end = Vector2::new(x, y);
        self.cutscene_motion = Some((end, start));
    }

    /// Calculates the target position relative to the current position
    #[func]
    pub fn move_to_relative(&mut self, x: real, y: real) {
        // TODO: this and `move_to_absolute` MUST check if cutscene_motion is
        // `Some` and if so, immediately jump to its target before overwriting it
        // Additionally, `move_to_relative` should teleport BEFORE getting the
        // starting position (aka above this comment, not below it)
        let start = self.base().get_global_position();
        let end = Vector2::new(x, y);
        let total = start + end;
        self.cutscene_motion = Some((total, start));
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
            || self.cutscene_motion.is_some()
            || self.in_cutscene;

        !cant_move
    }

    pub fn is_in_battle(&self) -> bool {
        !si().bind().battling.is_empty()
    }

    /// Set character positions based on past pos/rot
    pub fn move_chars(&mut self, moving: bool) {
        // don't run if disabled or if no previous positions saved
        if self.in_cutscene || self.past_positions.is_empty() {
            return;
        }

        for (i, mut ch_node) in self.party.iter_shared().enumerate() {
            // index of past data to get from the `LimiQ`s
            let nth = i * PERSONAL_SPACE;
            ch_node.set_global_position(*self.past_positions.get_or_last(nth));

            let mut ch = ch_node.bind_mut();
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
    fn calc_movements(&mut self, inputs: Inputs, delta: f64) -> bool {
        let Inputs {
            input_vector,
            sprinting,
        } = inputs;

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
            .set_velocity(velocity.move_toward(target_pos, deltatimes));

        self.base_mut().move_and_slide();

        let pos_updated = (self.past_positions.is_empty())
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
        self.past_rotations
            .front()
            .cloned()
            .unwrap_or(Vector2::ZERO)
    }

    #[func]
    pub fn wipe_party(&mut self, delete_pcharnodes: bool) {
        if delete_pcharnodes {
            // queue_free the pcharnodes first, while we still hold references
            for mut pcharnode in self.party.iter_shared() {
                pcharnode.queue_free();
            }
        }

        self.party.clear();
    }

    /// Push 1 character to the party.
    /// Accepts [GString] and is marked with #[func]
    #[func]
    fn push_pchar_gd(&mut self, name: GString) -> Gd<PCharNode> {
        // because godot can't understand `impl Trait`
        self.push_pchar(name.to_string())
    }

    /// Push 1 character to the party.
    pub fn push_pchar(&mut self, name: impl ToString) -> Gd<PCharNode> {
        let path = format!("res://scenes/char/{}.tscn", name.to_string());
        let packed = load::<PackedScene>(&path);
        let inst = packed.instantiate_as::<PCharNode>();
        self.base_mut().add_child(&inst);
        self.party.push(&inst);
        inst
    }
}

#[godot_api]
impl ICharacterBody2D for PartyCB {
    fn physics_process(&mut self, delta: f64) {
        let mut moving = false;

        if self.can_move() {
            let inputs = Inputs::from_player_input();
            moving = self.calc_movements(inputs, delta);
        } else if let Some((target, old_pos)) = self.cutscene_motion {
            let own_pos = self.base().get_global_position();
            let input_vector = Inputs::iv_from_to(own_pos, target);

            moving = self.calc_movements(
                Inputs {
                    input_vector,
                    sprinting: false, // TODO
                },
                delta,
            );

            if (target - own_pos).length() < CUTSCENE_MOTION_CLOSE_ENOUGH {
                self.cutscene_motion = None;
                self.base_mut().emit_signal("pcb_motion_done", &[]);
                self.base_mut().set_global_position(target);
            }
        }

        self.move_chars(moving);
    }
}
