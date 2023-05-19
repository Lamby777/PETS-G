extends Node2D

@export var world_music	:= ""

@onready var mzones			= $MusicZones
@onready var zoneaudio		= $ZoneAudio
@onready var zoneaudiofade	= $ZoneAudioFade
@onready var player			= $YSort/PlayerCB

var current_mzone: MusicZone = null

func _process(_delta):
	check_musiczones()

func check_musiczones():
	# check if entering new zone
	for zone in mzones.get_children():
		zone = zone as MusicZone # type hinting uwu
		
		var is_in_zone = Geometry2D.is_point_in_polygon(player.position, zone.polygon)
		
		# if leaving current zone
		if zone == current_mzone and not is_in_zone:
			fade_out_current_mz()
		
		# if entering new zone
		if not zone == current_mzone and is_in_zone:
			print("Entering new MusicZone: " + zone.name)
			current_mzone = zone
			zoneaudio.stream = zone.music
			zoneaudio.play()
			# not able to break anymore, since might skin current zone
			# maybe forego readability here later for performance reasons?

func fade_out_current_mz():
	current_mzone = null
	#zoneaudio.
