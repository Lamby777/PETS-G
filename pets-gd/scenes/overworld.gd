extends Node2D

@onready var mzones		= $MusicZones
@onready var za_active	= $ZoneAudio/Active
@onready var za_fade	= $ZoneAudio/FadeOut
@onready var za_anim	= $ZoneAudio/AnimationPlayer
@onready var player		= $YSort/PlayerCB

var current_mz: MusicZone = null

func _process(_delta):
	check_musiczones()

func check_musiczones():
	# check if entering new zone
	for zone in mzones.get_children():
		zone = zone as MusicZone # type hinting uwu
		
		var is_in_zone = Geometry2D.is_point_in_polygon(player.position, zone.polygon)
		
		# if leaving current zone
		if zone == current_mz and not is_in_zone:
			crossfade_za_into(null)
			current_mz = null
		
		# if entering a new zone
		if not zone == current_mz and is_in_zone:
			print("Entering new MusicZone: " + zone.name)
			crossfade_za_into(zone.music)
			current_mz = zone
			
			# not able to break anymore, since might skip current zone
			# maybe forego readability here later for performance reasons?
			
			# update: fade out is gonna freak out if this doesn't break,
			# so we prob should make it break again...

func crossfade_za_into(new_audio: AudioStream):
	# before assigning a new stream, keep track of where
	# the old one ended on, to assign the fadeout's pos to that
	var fadeout_at		= za_active.get_playback_position()

	za_fade.stream		= za_active.stream
	za_active.stream	= new_audio
	
	# just for testing
	# use a value provided by the mz later on...
	za_anim.speed_scale = 0.2
	
	za_anim.play("crossfade")
	
	za_active.playing = true
	za_fade.play(fadeout_at)
