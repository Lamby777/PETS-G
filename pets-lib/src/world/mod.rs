//!
//! Overworld Stuff
//!

use crate::prelude::*;
use godot::prelude::*;

pub mod enemy_node;
pub mod interaction;
pub mod music_zone;
pub mod pchar_node;
pub mod playercb;

use music_zone::MusicZone;

// @onready var za_active = $ZoneAudio/Active
// @onready var za_fade   = $ZoneAudio/FadeOut
// @onready var za_anim   = $ZoneAudio/AnimationPlayer
// @onready var player    = $YSort/PlayerCB
//
// var current_mz: MusicZone = null
//
// func leaving_mz(cb):
//     if not (cb is PlayerCB): return
//     crossfade_za_into_null()
//
// func entering_mz(cb, zone):
//     if not (cb is PlayerCB): return
//
//     print("Entering new MusicZone: " + zone.name)
//     crossfade_za_into(zone.music)
//     current_mz = zone
//
// func crossfade_za_into_null():
//     crossfade_za_into(null)
//     current_mz = null
//
// func crossfade_za_into(new_audio: AudioStream):
//     # before assigning a new stream, keep track of where
//     # the old one ended on, to assign the fadeout's pos to that
//     var fadeout_at    = za_active.get_playback_position()
//
//     za_fade.stream    = za_active.stream
//     za_active.stream  = new_audio
//
//     # just for testing
//     # use a value provided by the mz later on...
//     za_anim.speed_scale = 0.5
//
//     za_anim.stop()
//     za_anim.play("crossfade")
//
//     za_active.playing = true
//     za_fade.play(fadeout_at)

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct World {
    base: Base<Node2D>,

    #[init(default = onready_node(&base, "YSort/Room"))]
    room: OnReady<Gd<Node2D>>,
}

#[godot_api]
impl World {
    //
}

#[godot_api]
impl INode2D for World {
    fn ready(&mut self) {
        let room = self.room.clone();
        let mzones = subchildren_of_type::<MusicZone>(room.upcast());

        for zone in mzones {
            todo!();
            // zone.body_entered.connect(entering_mz.bind(zone))
            // zone.body_exited.connect(leaving_mz)
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        //
    }
}
