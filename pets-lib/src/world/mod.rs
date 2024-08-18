//!
//! Overworld Stuff
//!

use crate::consts::battle::*;
use crate::prelude::*;

use enemy_node::WalkingEnemy;
use godot::classes::{AnimationPlayer, AudioServer, AudioStream, CanvasLayer};
use godot::global::randf_range;
use godot::prelude::*;

mod enemy_node;
mod interaction;
mod inv_node;
mod menu; // mod menu?? are you hacking?!!! ban ban report >:3
mod pchar_node;
mod playercb;

mod music_zone;
mod water_zone;
use music_zone::MusicZone;
use water_zone::WaterZone;

pub use interaction::{InteractionManager, InteractionZone};
pub use menu::WorldMenu;
pub use playercb::PlayerCB;

// just for testing
// use a value provided by the mz later on...
const AUDIO_FADE_TIME: real = 0.5;

fn set_or_stop_audio(
    src: Option<Gd<AudioStream>>,
    mut audio: Gd<AudioStreamPlayer>,
) {
    match src {
        Some(src) => audio.set_stream(src),
        None => audio.stop(),
    }
}

/// randomize vector both fields from -1.0 to 1.0
fn generate_random_mod() -> Vector2 {
    let generate = || randf_range(-1.0, 1.0) as f32;
    Vector2::new(generate(), generate())
}

fn cue_battle_intro_fx() {
    // start the cool shader rectangle thing
    let mut rect = PlayerCB::fx_rect();
    let mut mat = PlayerCB::fx_material();
    rect.call("reset_shader_timer".into(), &[]);

    let rand_mod = generate_random_mod().to_variant();
    mat.set_shader_parameter("rand_mod".into(), rand_mod);

    rect.set_visible(true);
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct World {
    base: Base<Node2D>,

    #[init(node = "ZoneAudio/Active")]
    active_audio: OnReady<Gd<AudioStreamPlayer>>,

    #[init(node = "ZoneAudio/FadeOut")]
    fading_audio: OnReady<Gd<AudioStreamPlayer>>,

    #[init(node = "ZoneAudio/AnimationPlayer")]
    fade_animator: OnReady<Gd<AnimationPlayer>>,

    current_mz: Option<Gd<MusicZone>>,

    #[export]
    battle_scene: Option<Gd<PackedScene>>,
}

// Due to a gdext limitation, you can only have 1 `#[godot_api]` custom `impl` block.
// There's gonna be a LOT of methods ahead, so look for these comment markers that
// split the code into sections.

#[godot_api]
impl World {
    /// Get the world node, or panic if not currently in the world scene.
    #[func]
    pub fn singleton() -> Gd<Self> {
        current_scene()
            .try_cast::<Self>()
            .expect("not in the world scene")
    }

    // ---------------------------------------- Room stuff

    pub fn room() -> Gd<Node2D> {
        World::singleton().get_node_as("YSort/Room")
    }

    pub fn change_room(&mut self, new_room: Gd<Node2D>) {
        let mut old_room = Self::room();

        for mut child in old_room.get_children().iter_shared() {
            old_room.remove_child(child.clone());
            child.queue_free();
        }

        old_room.replace_by(&new_room);

        let mut world = self.base_mut();
        world.call_deferred("reconnect_musiczones".into(), &[]);
        world.call_deferred("reconnect_waterzones".into(), &[]);
    }

    // ---------------------------------------- Battle stuff

    #[signal]
    fn battle_intro_done(eid: GString) {}

    fn mute_audio_bus(mute_world: bool) {
        let (muted, unmuted) = if mute_world { (1, 2) } else { (2, 1) };

        let mut srv = AudioServer::singleton();
        srv.set_bus_mute(muted, true);
        srv.set_bus_mute(unmuted, false);
    }

    #[func]
    pub fn start_battle(eid: GString) {
        let eid = EnemyID::from_godot(eid);
        let enemy_data = EnemyData::from_id(eid);
        pcb()
            .bind_mut()
            .battling
            .push(Rc::new(RefCell::new(enemy_data)));
        let world = World::singleton();

        let mat = PlayerCB::fx_material();
        let fade_len = mat.get_shader_parameter("LENGTH".into()).to::<f64>();

        set_timeout(INTRO_FADE_PREDELAY, cue_battle_intro_fx);

        let cue_scene = world
            .callable("cue_battle_scene")
            .bindv((&[eid.to_variant()]).into());
        set_timeout_callable(INTRO_FADE_PREDELAY + fade_len, cue_scene);
    }

    fn instantiate_battle_scene(&self) -> Gd<BattleEngine> {
        self.battle_scene
            .as_ref()
            .expect("no battle scene provided in exported field")
            .instantiate_as()
    }

    #[func]
    fn cue_battle_scene(&mut self, _eid: GString) {
        // emit a signal for other nodes if they need to do something
        self.base_mut()
            .emit_signal("battle_intro_done".into(), &[_eid.to_variant()]);

        let mut layer = self.base().get_node_as::<CanvasLayer>(LAYER_NAME);

        // load the scene
        let mut scene = self.instantiate_battle_scene();

        layer.add_child(&scene);
        scene.bind_mut().animate_in();

        Self::mute_audio_bus(true);

        // it's a performance thing
        pcb().set_process(false);
        children_of_type::<WalkingEnemy, _>(&self.to_gd());
    }

    // ---------------------------------------- MusicZone stuff

    #[func]
    fn on_mz_exit(&mut self, _pcb: Gd<Node2D>) {
        self.crossfade_audio_to_null();
    }

    #[func]
    fn on_mz_enter(&mut self, _pcb: Gd<Node2D>, zone: Gd<MusicZone>) {
        godot_print!("Entering new MusicZone: {}", zone.get_name());
        self.crossfade_audio_into(zone.bind().music.clone());
        self.current_mz = Some(zone);
    }

    #[func]
    fn crossfade_audio_into(&mut self, src: Option<Gd<AudioStream>>) {
        // before assigning a new stream, keep track of where
        // the old one ended on, to assign the fadeout's pos to that
        let fadeout_at = self.active_audio.get_playback_position();

        let old_stream = self.active_audio.get_stream();
        set_or_stop_audio(old_stream, self.fading_audio.clone());
        set_or_stop_audio(src, self.active_audio.clone());

        // TODO maybe if there's a way to "reverse" the
        // animation from the current point...? that would
        // easily solve <https://github.com/Lamby777/PETS-G/issues/9>
        self.fade_animator.set_speed_scale(AUDIO_FADE_TIME);
        self.fade_animator.stop();

        self.fade_animator
            .set_assigned_animation("crossfade".into());
        self.fade_animator.play();

        // play the stuff
        self.active_audio.play();
        self.fading_audio.seek(fadeout_at);
        self.fading_audio.play();
    }

    #[func]
    pub fn crossfade_audio_to_null(&mut self) {
        self.crossfade_audio_into(None);
        self.current_mz = None;
        // TODO set current_mz once again once the battle is over
    }

    // ---------------------------------------- WaterZone stuff

    #[func]
    pub fn on_water_enter(&mut self, _pcb: Gd<Node2D>, zone: Gd<WaterZone>) {
        let mut pcb = pcb();
        pcb.bind_mut().in_water = true;
        pcb.bind_mut().water_speed_mod = zone.bind().speed_modulation;
    }

    #[func]
    pub fn on_water_exit(&mut self, _pcb: Gd<Node2D>) {
        let mut pcb = pcb();
        pcb.bind_mut().in_water = false;
        pcb.bind_mut().water_speed_mod = 1.0;
    }

    // ---------------------------------------- General zone stuff

    #[func]
    fn reconnect_musiczones(&mut self) {
        let room = Self::room();
        let mzones = children_of_type::<MusicZone, _>(&room);

        for mut zone in mzones {
            disconnect_signal(&mut zone, "body_entered");
            disconnect_signal(&mut zone, "body_exited");

            let on_mz_exit = self.base().callable("on_mz_exit");
            let on_mz_enter = self
                .base()
                .callable("on_mz_enter")
                .bindv((&[zone.to_variant()]).into());

            zone.connect("body_entered".into(), on_mz_enter);
            zone.connect("body_exited".into(), on_mz_exit);
        }
    }

    #[func]
    fn reconnect_waterzones(&mut self) {
        let room = Self::room();
        let wzones = children_of_type::<WaterZone, _>(&room);

        for mut zone in wzones {
            disconnect_signal(&mut zone, "body_entered");
            disconnect_signal(&mut zone, "body_exited");

            let on_water_exit = self.base().callable("on_water_exit");
            let on_water_enter = self
                .base()
                .callable("on_water_enter")
                .bindv((&[zone.to_variant()]).into());

            zone.connect("body_entered".into(), on_water_enter);
            zone.connect("body_exited".into(), on_water_exit);
        }
    }
}

#[godot_api]
impl INode2D for World {
    fn ready(&mut self) {
        self.reconnect_musiczones();
        self.reconnect_waterzones();
    }
}
