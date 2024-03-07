use godot::engine::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, IStaticBody2D, Sprite2D,
    StaticBody2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct WalkingEnemy {
    base: Base<StaticBody2D>,
    sprite: OnReady<Gd<Sprite2D>>,
    anim_player: OnReady<Gd<AnimationPlayer>>,
    anim_tree: OnReady<Gd<AnimationTree>>,
    anim_state: OnReady<Gd<AnimationNodeStateMachinePlayback>>,
}

#[godot_api]
impl WalkingEnemy {
    #[func]
    pub fn anim_move(&mut self, moving: bool, inputs: Vector2) {
        let mode_str = if moving { "Run" } else { "Idle" };
        let anim_path = format!("parameters/{mode_str}/blend_position");

        self.anim_tree.set(anim_path.into(), Variant::from(inputs));
        self.anim_state.travel(mode_str.into());
    }
}

#[godot_api]
impl IStaticBody2D for WalkingEnemy {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            sprite: OnReady::manual(),
            anim_player: OnReady::manual(),
            anim_tree: OnReady::manual(),
            anim_state: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let sprite = self.base().get_node_as("Sprite2D");
        let anim_player = self.base().get_node_as("AnimationPlayer");
        self.sprite.init(sprite);
        self.anim_player.init(anim_player);

        let mut tree = self.base().get_node_as::<AnimationTree>("AnimationTree");
        tree.set_active(true);
        let anim_state = tree.get("parameters/playback".into()).to();
        self.anim_state.init(anim_state);

        self.anim_tree.init(tree);
    }

    fn physics_process(&mut self, _delta: f64) {
        // walk towards player
        todo!()
    }
}
