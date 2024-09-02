use godot::builtin::math::ApproxEq;
use godot::classes::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, Area2D,
    Sprite2D,
};
use godot::prelude::*;

use super::partycb::Inputs;
use crate::common::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct PCharNode {
    base: Base<Node2D>,

    #[export]
    #[init(val = PChar::Devon)]
    pub pchar: PChar,

    #[init(node = "Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,

    #[init(node = "AnimationPlayer")]
    anim_player: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "AnimationTree")]
    anim_tree: OnReady<Gd<AnimationTree>>,

    #[init(node = "Area2D")]
    area: OnReady<Gd<Area2D>>,

    #[init(val = OnReady::manual())]
    anim_state: OnReady<Gd<AnimationNodeStateMachinePlayback>>,

    last_position: Vector2,
}

#[godot_api]
impl PCharNode {
    #[signal]
    fn motion_done(&self);

    #[func]
    pub fn anim_move(&mut self) {
        let cur_pos = self.base().get_global_position();
        let moving = self.last_position != cur_pos;
        let inputs = Inputs::iv_from_to(self.last_position, cur_pos);
        self.last_position = cur_pos;

        // change the animationtree state machine to the correct mode
        let mode_str = self.anim_mode_str(moving);
        self.anim_state.travel(mode_str.into());

        // set the blend position
        if !inputs.approx_eq(&Vector2::ZERO) {
            self.set_blend_positions(inputs);
        }
    }

    fn set_blend_positions(&mut self, inputs: Vector2) {
        for mode in &["Idle", "Run", "Wade"] {
            let blend_pos_field = format!("parameters/{mode}/blend_position");
            self.anim_tree
                .set(blend_pos_field.into(), Variant::from(inputs));
        }
    }

    fn anim_mode_str(&self, moving: bool) -> &'static str {
        match moving {
            _ if self.overlapping_water() => "Wade",
            true => "Run",
            false => "Idle",
        }
    }

    fn overlapping_water(&self) -> bool {
        let overlapping_areas = self.area.get_overlapping_areas();
        overlapping_areas
            .iter_shared()
            .any(|area| area.is_in_group("water".into()))
    }
}

#[godot_api]
impl INode2D for PCharNode {
    fn ready(&mut self) {
        self.anim_tree.set_active(true);
        let anim_state = self.anim_tree.get("parameters/playback".into()).to();
        self.anim_state.init(anim_state);
    }

    fn physics_process(&mut self, _delta: f64) {
        // let mut moving;
        //
        // let own_pos = self.base().get_global_position();
        // let input_vector = Inputs::iv_from_to(own_pos, target);
        //
        // self.base_mut().set_global_position(
        //     own_pos + input_vector * MAX_SPEED * delta as real,
        // );
        //
        // moving = true;
        //
        // if (target - own_pos).length() < CUTSCENE_MOTION_CLOSE_ENOUGH {
        //     self.cutscene_motion = None;
        //     self.base_mut().emit_signal("motion_done".into(), &[]);
        //     self.base_mut().set_global_position(target);
        //     moving = false;
        // }
        //
        // self.anim_move(moving, input_vector);
    }
}
