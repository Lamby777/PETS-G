//!
//! Overworld Stuff
//!

use crate::prelude::*;

use godot::engine::AnimationPlayer;
use godot::prelude::*;

pub mod enemy_node;
pub mod interaction;
pub mod music_zone;
pub mod pchar_node;
pub mod playercb;

use music_zone::MusicZone;

// just for testing
// use a value provided by the mz later on...
const AUDIO_FADE_TIME: f64 = 0.5;

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

    current_mz: Option<MusicZone>,
}
// func leaving_mz(cb):
//     crossfade_za_into_null()
//
// func entering_mz(cb, zone):
//     print("Entering new MusicZone: " + zone.name)
//     crossfade_za_into(zone.music)
//     current_mz = Some(zone)
//
// func crossfade_za_into(new_audio: AudioStream):
//     # before assigning a new stream, keep track of where
//     # the old one ended on, to assign the fadeout's pos to that
//     var fadeout_at    = active_audio.get_playback_position()
//
//     fading_audio.stream = active_audio.stream
//     active_audio.stream = new_audio
//
//     fade_animator.speed_scale = AUDIO_FADE_TIME
//
//     fade_animator.stop()
//     fade_animator.play("crossfade")
//
//     active_audio.playing = true
//     fading_audio.play(fadeout_at)

#[godot_api]
impl World {
    #[func]
    fn crossfade_za_to_null(&mut self) {
        // self.crossfade_za_into(null);
        self.current_mz = None;
    }
}

#[godot_api]
impl INode2D for World {
    fn ready(&mut self) {
        let room = self.room.clone();
        let mzones = subchildren_of_type::<MusicZone>(room.upcast());

        for mut zone in mzones {
            let on_exit = self.base().callable("crossfade_za_to_null");

            // zone.body_entered.connect(entering_mz.bind(zone))
            zone.connect("body_exited".into(), on_exit);
        }
    }
}
