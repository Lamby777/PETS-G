// @onready var anim_state   = anim_tree["parameters/playback"]
//
// func anim_move(moving: bool, inputs: Vector2):
//   anim_tree.set("parameters/Idle/blend_position", inputs)
//   anim_tree.set("parameters/Run/blend_position", inputs)
//
//   if moving:
//     anim_state.travel("Run")
//   else:
//     anim_state.travel("Idle")
//

use godot::engine::{AnimationPlayer, AnimationTree, Sprite2D};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PCharNode {
    #[base]
    node: Base<Node2D>,

    sprite: Gd<Sprite2D>,
    anim_player: Gd<AnimationPlayer>,
    anim_tree: Gd<AnimationTree>,
}

#[godot_api]
impl Node2DVirtual for PCharNode {
    fn ready(&mut self) {
        self.sprite = self.node.get_node_as("Sprite2D");

        self.anim_tree.active = true;
    }
}
