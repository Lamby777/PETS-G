use godot::engine::AnimationNodeStateMachinePlayback;
use godot::engine::AnimationPlayer;
use godot::engine::AnimationTree;
use godot::engine::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct PCharNode {
    base: Base<Node2D>,

    #[init(default = OnReady::manual())]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(default = OnReady::manual())]
    anim_player: OnReady<Gd<AnimationPlayer>>,
    #[init(default = OnReady::manual())]
    anim_tree: OnReady<Gd<AnimationTree>>,
    #[init(default = OnReady::manual())]
    anim_state: OnReady<Gd<AnimationNodeStateMachinePlayback>>,
}

#[godot_api]
impl PCharNode {
    #[func]
    pub fn anim_move(&mut self, moving: bool, inputs: Vector2) {
        let mode_str = if moving { "Run" } else { "Idle" };
        let anim_path = format!("parameters/{mode_str}/blend_position");

        self.anim_tree.set(anim_path.into(), Variant::from(inputs));
        self.anim_state.travel(mode_str.into());
    }
}

#[macro_export]
macro_rules! load_pchar_scenes_under {
    ($parent:expr; $($name:expr),*) => {{
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
        let path = concat!("res://scenes/char/", $name, ".tscn");
        let packed = load::<PackedScene>(path);
        let inst = packed.instantiate_as::<PCharNode>();
        $parent.base_mut().add_child(inst.clone().upcast());
        inst
    }};
}

#[godot_api]
impl INode2D for PCharNode {
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
}
