use godot::engine::AnimationNodeStateMachinePlayback;
use godot::engine::AnimationPlayer;
use godot::engine::AnimationTree;
use godot::engine::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct PCharNode {
    #[base]
    node: Base<Node2D>,

    sprite: Gd<Sprite2D>,
    anim_player: Gd<AnimationPlayer>,
    anim_tree: Gd<AnimationTree>,
    anim_state: Gd<AnimationNodeStateMachinePlayback>,
}

#[godot_api]
impl PCharNode {
    fn anim_move(&mut self, moving: bool, inputs: Vector2) {
        let mode_str = if moving { "Run" } else { "Idle" };
        let anim_path = format!("parameters/{mode_str}/blend_position");

        self.anim_tree.set(anim_path.into(), Variant::from(inputs));
        self.anim_state.travel(mode_str.into());
    }
}

#[godot_api]
impl Node2DVirtual for PCharNode {
    fn ready(&mut self) {
        self.sprite = self.node.get_node_as("Sprite2D");
        self.anim_player = self.node.get_node_as("AnimationPlayer");
        self.anim_tree = self.node.get_node_as("AnimationTree");
        self.anim_state = self.anim_tree.get("parameters/playback".into()).to();

        self.anim_tree.set_active(true);
    }
}
