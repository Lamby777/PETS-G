//!
//! Overworld Stuff
//!

use crate::prelude::*;

use godot::engine::utilities::{randf, randf_range, randi};
use godot::engine::{AnimationPlayer, AudioStream, ShaderMaterial, Time};
use godot::prelude::*;

pub mod enemy_node;
pub mod interaction;
pub mod music_zone;
pub mod pchar_node;
pub mod playercb;

pub use interaction::manager::InteractionManager;
pub use interaction::zone::InteractionZone;
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
}

fn set_or_stop_audio(src: Option<Gd<AudioStream>>, mut audio: Gd<AudioStreamPlayer>) {
    match src {
        Some(src) => audio.set_stream(src),
        None => audio.stop(),
    }
}

fn program_time() -> u64 {
    // let time = Time::singleton();
    // let t = time.get_ticks_msec();
    // t - (t.div_floor(3600)) * 3600

    Time::singleton().get_ticks_msec()
}

/// randomize vector both fields from -1.0 to 1.0
fn generate_random_mod() -> Vector2 {
    let generate = || randf_range(-1.0, 1.0) as f32;
    Vector2::new(generate(), generate())
}

#[godot_api]
impl World {
    fn battle_start(_eid: GString) {
        let mut fx_rect = PlayerCB::singleton().bind().get_fx_rect();
        let mut mat = fx_rect.get_material().unwrap().cast::<ShaderMaterial>();

        let ptime = (program_time() as real).to_variant();
        mat.set_shader_parameter("start_time".into(), ptime);

        let rand_mod = generate_random_mod().to_variant();
        mat.set_shader_parameter("rand_mod".into(), rand_mod);

        fx_rect.set_visible(true);
    }

    #[func]
    fn on_exit(&mut self, _pcb: Gd<Node2D>) {
        self.crossfade_audio_to_null();
    }

    #[func]
    fn on_enter(&mut self, _pcb: Gd<Node2D>, zone: Gd<MusicZone>) {
        godot_print!("Entering new MusicZone: {}", zone.get_name());
        self.crossfade_audio_into(Some(zone.bind().music.clone()));
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
    fn crossfade_audio_to_null(&mut self) {
        self.crossfade_audio_into(None);
        self.current_mz = None;
    }
}

#[godot_api]
impl INode2D for World {
    fn ready(&mut self) {
        let room = self.room.clone();
        let mzones = subchildren_of_type::<MusicZone>(room.upcast());

        for mut zone in mzones {
            let on_exit = self.base().callable("on_exit");
            let on_enter = self.base().callable("on_enter");

            let args = Array::from(&[zone.to_variant()]);
            zone.connect("body_entered".into(), on_enter.bindv(args));
            zone.connect("body_exited".into(), on_exit);
        }
    }
}
