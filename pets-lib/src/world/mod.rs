//!
//! Overworld Stuff
//!

use crate::prelude::*;

use godot::engine::{AnimationPlayer, AudioStream};
use godot::prelude::*;

pub mod enemy_node;
pub mod interaction;
pub mod music_zone;
pub mod pchar_node;
pub mod playercb;

use music_zone::MusicZone;

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

#[godot_api]
impl World {
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
    fn crossfade_audio_into(&mut self, _new_audio: Option<Gd<AudioStream>>) {
        // before assigning a new stream, keep track of where
        // the old one ended on, to assign the fadeout's pos to that
        let _fadeout_at = self.active_audio.get_playback_position();
        // fading_audio.stream = active_audio.stream
        // active_audio.stream = new_audio
        //
        self.fade_animator.set_speed_scale(AUDIO_FADE_TIME);
        self.fade_animator.stop();

        self.fade_animator
            .set_assigned_animation("crossfade".into());
        self.fade_animator.play()

        // active_audio.playing = true
        // fading_audio.play(fadeout_at)
    }

    #[func]
    fn crossfade_audio_to_null(&mut self) {
        // self.crossfade_audio_into(null);
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
