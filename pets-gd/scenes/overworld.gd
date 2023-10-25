extends Node2D

@onready var mzones    = $MusicZones
@onready var za_active = $ZoneAudio/Active
@onready var za_fade   = $ZoneAudio/FadeOut
@onready var za_anim   = $ZoneAudio/AnimationPlayer
@onready var player    = $YSort/PlayerCB

var current_mz: MusicZone = null

func _ready():
  # check if entering new zone
  for zone in mzones.get_children():
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

signal register_inter(inter: InteractionZone)
signal unregister_inter(inter: InteractionZone)
