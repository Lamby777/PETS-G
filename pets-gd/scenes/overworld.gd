extends Node2D

@onready var za_active = $ZoneAudio/Active
@onready var za_fade   = $ZoneAudio/FadeOut
@onready var za_anim   = $ZoneAudio/AnimationPlayer
@onready var player    = $YSort/PlayerCB
@onready var room      = $YSort/Room

var current_mz: MusicZone = null

func get_subchildren_of_type(type, parent) -> Array:
    var res: Array = []
    
    for node in parent.get_children():
        if is_instance_of(node, type):
            res.append(node)
        elif node.get_child_count() > 0:
            res.append_array(get_subchildren_of_type(type, node))

    return res

func _ready():
    var mzones = get_subchildren_of_type(MusicZone, room)
    # check if entering new zone
    for zone in mzones:
        zone.body_entered.connect(entering_mz.bind(zone))
        zone.body_exited.connect(leaving_mz)

func leaving_mz(cb):
    if not (cb is PlayerCB): return
    crossfade_za_into_null()

func entering_mz(cb, zone):
    if not (cb is PlayerCB): return
    
    print("Entering new MusicZone: " + zone.name)
    crossfade_za_into(zone.music)
    current_mz = zone

func crossfade_za_into_null():
    crossfade_za_into(null)
    current_mz = null

func crossfade_za_into(new_audio: AudioStream):
    # before assigning a new stream, keep track of where
    # the old one ended on, to assign the fadeout's pos to that
    var fadeout_at    = za_active.get_playback_position()

    za_fade.stream    = za_active.stream
    za_active.stream  = new_audio
    
    # just for testing
    # use a value provided by the mz later on...
    za_anim.speed_scale = 0.5
    
    za_anim.stop()
    za_anim.play("crossfade")
    
    za_active.playing = true
    za_fade.play(fadeout_at)
