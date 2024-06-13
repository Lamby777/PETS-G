use godot::engine::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, Sprite2D,
};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct PCharNode {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "Sprite2D"))]
    sprite: OnReady<Gd<Sprite2D>>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    anim_player: OnReady<Gd<AnimationPlayer>>,

    #[init(default = onready_node(&base, "AnimationTree"))]
    anim_tree: OnReady<Gd<AnimationTree>>,

    #[init(default = OnReady::manual())]
    anim_state: OnReady<Gd<AnimationNodeStateMachinePlayback>>,
}

#[godot_api]
impl PCharNode {
    #[func]
    pub fn anim_move(&mut self, moving: bool, inputs: Vector2) {
        let mode_str = self.anim_mode_str(moving);
        let anim_path = format!("parameters/{mode_str}/blend_position");

        self.anim_tree.set(anim_path.into(), Variant::from(inputs));
        self.anim_state.travel(mode_str.into());
    }

    fn anim_mode_str(&self, moving: bool) -> &'static str {
        // if true {
        //     return "Wade";
        // }

        match moving {
            true => "Run",
            false => "Idle",
        }
    }
}

#[macro_export]
macro_rules! load_pchar_scenes_under {
    ($parent:expr; $($name:expr),* $(,)?) => {{
        let mut res = vec![];

        $({
            res.push($crate::load_pchar_scene_under!($parent, $name));
        })*

        res
    }};
}

#[macro_export]
macro_rules! load_pchar_scene_under {
    ($parent:expr, $name:expr) => {{
        let path = format!("res://scenes/char/{}.tscn", $name);
        let packed = load::<PackedScene>(path);
        let inst = packed.instantiate_as::<PCharNode>();
        $parent.base_mut().add_child(inst.clone().upcast());
        inst
    }};
}

#[godot_api]
impl INode2D for PCharNode {
    fn ready(&mut self) {
        self.anim_tree.set_active(true);
        let anim_state = self.anim_tree.get("parameters/playback".into()).to();
        self.anim_state.init(anim_state);
    }
}
