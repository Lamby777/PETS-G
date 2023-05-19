extends Node2D

@export var world_music	:= ""

@onready var mzones	= $MusicZones

func _ready():
	check_musiczones()

func check_musiczones():
	# check if leaving current zone
	#
	#
	#
	#
	
	# check if entering new zone
	for zone in mzones.get_children():
		if Geometry2D.is_point_in_polygon(position, zone.polygon):
			var world_music = zone.get_meta("music")
			break
