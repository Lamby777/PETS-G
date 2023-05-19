extends Node2D

@onready var mzones		= $MusicZones
@onready var za_active	= $ZoneAudio/Active
@onready var za_fade	= $ZoneAudio/FadeOut
@onready var za_anim	= $ZoneAudio/AnimationPlayer
@onready var player		= $YSort/PlayerCB

var current_mz: MusicZone = null

func _ready():
	za_active.play()
	za_fade.play()

func _process(_delta):
	check_musiczones()

func check_musiczones():
	# check if entering new zone
	for zone in mzones.get_children():
		zone = zone as MusicZone # type hinting uwu
		
		var is_in_zone = Geometry2D.is_point_in_polygon(player.position, zone.polygon)
		
		# if leaving current zone
		if zone == current_mz and not is_in_zone:
			crossfade_za_between(current_mz.music, null)
			current_mz = null
		
		# if entering a new zone
		if not zone == current_mz and is_in_zone:
			print("Entering new MusicZone: " + zone.name)
			crossfade_za_between(current_mz.music, zone.music)
			current_mz = zone
			
			# not able to break anymore, since might skip current zone
			# maybe forego readability here later for performance reasons?
			
			# update: fade out is gonna freak out if this doesn't break,
			# so we prob should make it break again...

func crossfade_za_between(
	old_audio: AudioStream,
	new_audio: AudioStream
):
	za_fade.stream		= old_audio
	za_active.stream	= new_audio
	za_anim.play("crossfade")
