//!
//! Overworld Stuff
//!

use crate::common::*;
use crate::consts::battle::*;

use enemy_node::WalkingEnemy;
use godot::classes::{
    AnimationPlayer, AudioServer, AudioStream, AudioStreamPlayer, CanvasLayer,
};
use godot::global::randf_range;
use godot::prelude::*;

mod enemy_node;
mod interaction;
mod inv_node;
mod menu; // mod menu?? are you hacking?!!! ban ban report >:3
mod partycb;
mod pchar_node;

mod music_zone;
mod water_zone;
use music_zone::MusicZone;
use water_zone::WaterZone;

pub use interaction::InteractionZone;
pub use partycb::PartyCB;

// just for testing
// use a value provided by the mz later on...
const AUDIO_FADE_TIME: real = 0.5;

fn set_or_stop_audio(
    src: Option<Gd<AudioStream>>,
    mut audio: Gd<AudioStreamPlayer>,
) {
    match src {
        Some(ref src) => audio.set_stream(src),
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
    let mut rect = PartyCB::fx_rect();
    let mut mat = PartyCB::fx_material();
    rect.call("reset_shader_timer", &[]);

    let rand_mod = generate_random_mod().to_variant();
    mat.set_shader_parameter("rand_mod", &rand_mod);

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

// Due to a gdext limitation, you can only have 1 `#[godot_api]` custom `impl` block
// per class.
//
// There are gonna be a LOT of methods ahead, so look for these comment markers that
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

    pub fn get_beacon(id: impl Into<GString>) -> Gd<InteractionZone> {
        let id = id.into();

        // get all beacons in the room
        let all_beacons =
            children_of_type::<InteractionZone, _>(&World::singleton());

        // find the (first?) one with the matching id
        all_beacons
            .into_iter()
            .find(|v| v.bind().beacon_id == id)
            .unwrap_or_else(|| panic!("beacon id `{id}` not found"))
    }

    /// Free the old room and replace it with the new one
    pub fn change_room(&mut self, mut new_room: Gd<Node2D>) {
        let mut old_room = Self::room();
        old_room.set_name("RoomDeleted");
        new_room.set_name("Room");
        old_room.add_sibling(&new_room);
        old_room.queue_free();

        let mut world = self.base_mut();
        world.call_deferred("reconnect_musiczones", &[]);
        world.call_deferred("reconnect_waterzones", &[]);
    }

    // ---------------------------------------- Battle stuff

    #[signal]
    fn battle_intro_done(eid: GString);

    /// TODO: this needs comments whenever i once again understand what it's for
    fn mute_audio_bus(mute_world: bool) {
        let (muted, unmuted) = if mute_world { (1, 2) } else { (2, 1) };

        let mut srv = AudioServer::singleton();
        srv.set_bus_mute(muted, true);
        srv.set_bus_mute(unmuted, false);
    }

    #[func]
    pub fn start_battle(eid: StringName) {
        let enemy_data = EnemyData::from_registry(eid.clone());

        si().bind_mut().battling.push(enemy_data.clone());
        let world = World::singleton();

        let mat = PartyCB::fx_material();
        let fade_len = mat.get_shader_parameter("LENGTH").to::<f64>();

        set_timeout(INTRO_FADE_PREDELAY, cue_battle_intro_fx);

        let cue_scene = world.callable("cue_battle_scene").bindv(&varray![eid]);
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
            .emit_signal("battle_intro_done", &[_eid.to_variant()]);

        let mut layer = self.base().get_node_as::<CanvasLayer>(LAYER_NAME);

        // load the scene
        let mut scene = self.instantiate_battle_scene();

        layer.add_child(&scene);
        scene.bind_mut().animate_in();

        Self::mute_audio_bus(true);

        // it's a performance thing
        pcb().set_process(false);
        children_of_type::<WalkingEnemy, _>(&self.to_gd()); // ... what? isn't this a noop?
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

        // TODO: maybe if there's a way to "reverse" the
        // animation from the current point...? that would
        // easily solve <https://github.com/Lamby777/PETS-G/issues/9>
        self.fade_animator.set_speed_scale(AUDIO_FADE_TIME);
        self.fade_animator.stop();

        self.fade_animator.set_assigned_animation("crossfade");
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
        // TODO: set current_mz once again once the battle is over
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
            let on_mz_enter =
                self.base().callable("on_mz_enter").bindv(&varray![zone]);

            zone.connect("body_entered", &on_mz_enter);
            zone.connect("body_exited", &on_mz_exit);
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
            let on_water_enter =
                self.base().callable("on_water_enter").bindv(&varray![zone]);

            zone.connect("body_entered", &on_water_enter);
            zone.connect("body_exited", &on_water_exit);
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
