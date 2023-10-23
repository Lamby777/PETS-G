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

    sprite: Option<Gd<Sprite2D>>,
    anim_player: Option<Gd<AnimationPlayer>>,
    anim_tree: Option<Gd<AnimationTree>>,
    anim_state: Option<Gd<AnimationNodeStateMachinePlayback>>,
}

#[godot_api]
impl PCharNode {
    #[func]
    pub fn anim_move(&mut self, moving: bool, inputs: Vector2) {
        let mode_str = if moving { "Run" } else { "Idle" };
        let anim_path = format!("parameters/{mode_str}/blend_position");

        self.anim_tree
            .as_mut()
            .unwrap()
            .set(anim_path.into(), Variant::from(inputs));
        self.anim_state.as_mut().unwrap().travel(mode_str.into());
    }
}

#[godot_api]
impl Node2DVirtual for PCharNode {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,

            sprite: None,
            anim_player: None,
            anim_tree: None,
            anim_state: None,
        }
    }

    fn ready(&mut self) {
        self.sprite = Some(self.node.get_node_as("Sprite2D"));
        self.anim_player = Some(self.node.get_node_as("AnimationPlayer"));

        let mut tree = self.node.get_node_as::<AnimationTree>("AnimationTree");
        tree.set_active(true);
        self.anim_state = tree.get("parameters/playback".into()).to();

        self.anim_tree = Some(tree);
    }
}
