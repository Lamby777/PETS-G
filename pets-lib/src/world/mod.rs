//!
//! Overworld Stuff
//!

use crate::consts::battle::*;
use crate::prelude::*;

use godot::engine::utilities::randf_range;
use godot::engine::{AnimationPlayer, AudioServer, AudioStream, CanvasLayer};
use godot::prelude::*;

mod enemy_node;
mod interaction;
mod menu; // mod menu?? are you hacking?!!! ban ban report >:3
mod music_zone;
mod pchar_node;
mod playercb;

pub use interaction::{InteractionManager, InteractionZone};
pub use menu::WorldMenu;
pub use music_zone::MusicZone;
pub use playercb::PlayerCB;

// just for testing
// use a value provided by the mz later on...
const AUDIO_FADE_TIME: real = 0.5;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct World {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "YSort/Room"))]
    room: OnReady<Gd<Node2D>>,

    #[init(default = onready_node(&base, "ZoneAudio/Active"))]
    active_audio: OnReady<Gd<AudioStreamPlayer>>,

    #[init(default = onready_node(&base, "ZoneAudio/FadeOut"))]
    fading_audio: OnReady<Gd<AudioStreamPlayer>>,

    #[init(default = onready_node(&base, "ZoneAudio/AnimationPlayer"))]
    fade_animator: OnReady<Gd<AnimationPlayer>>,

    current_mz: Option<Gd<MusicZone>>,

    #[export]
    battle_scene: Option<Gd<PackedScene>>,
}

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

#[godot_api]
impl World {
    #[signal]
    fn battle_intro_done(eid: GString) {}

    fn mute_audio_bus(mute_world: bool) {
        let (muted, unmuted) = if mute_world { (1, 2) } else { (2, 1) };

        let mut srv = AudioServer::singleton();
        srv.set_bus_mute(muted, true);
        srv.set_bus_mute(unmuted, false);
    }

    pub fn start_battle(eid: GString) {
        PlayerCB::singleton().bind_mut().in_battle = true;
        let world = current_scene();

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

        layer.add_child(scene.clone().upcast());
        scene.bind_mut().animate_in();

        Self::mute_audio_bus(true);

        // it's a performance thing
        // PlayerCB::singleton().set_process(false);
        {
            // subchildren_of_type::<WalkingEnemy, _>(self.to_gd());
        }
    }

    #[func]
    fn on_exit(&mut self, _pcb: Gd<Node2D>) {
        self.crossfade_audio_to_null();
    }

    #[func]
    fn on_enter(&mut self, _pcb: Gd<Node2D>, zone: Gd<MusicZone>) {
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
}

#[godot_api]
impl INode2D for World {
    fn ready(&mut self) {
        let room = self.room.clone();
        let mzones = subchildren_of_type::<MusicZone, _>(room);

        for mut zone in mzones {
            let on_exit = self.base().callable("on_exit");
            let on_enter = self
                .base()
                .callable("on_enter")
                .bindv((&[zone.to_variant()]).into());

            zone.connect("body_entered".into(), on_enter);
            zone.connect("body_exited".into(), on_exit);
        }
    }
}
