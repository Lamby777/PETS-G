extends Node2D

@export var world_music	:= ""

@onready var mzones		= $MusicZones
@onready var zoneaudio	= $ZoneAudio
@onready var player		= $YSort/PlayerCB

func _process(_delta):
	check_musiczones()

func check_musiczones():
	# check if leaving current zone
	#
	#
	#
	#
	
	# check if entering new zone
	for zone in mzones.get_children():
		zone = zone as MusicZone # type hinting uwu
		
		if Geometry2D.is_point_in_polygon(player.position, zone.polygon):
			print("Entering new MusicZone: " + zone.name)
			zoneaudio.stream = zone.music
			zoneaudio.play()
			break
