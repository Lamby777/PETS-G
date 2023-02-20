extends Area2D

class_name Exit

"""
Add this to any area2d and it will send the player to the indicated scene and spawnpoint
"""

@export var to_scene = "" # (String, FILE, "*.tscn")
@export var spawnpoint: String = ""

# Called when the node enters the scene tree for the first time.
func _ready():
	# warning-ignore:return_value_discarded
	connect("body_entered",Callable(self,"_on_body_entered"))
	pass # Replace with function body.

func _on_body_entered(body):
	if body is Player:
		if  to_scene == "":
			push_error("Error changing scenes: to_scene has no assigned scene")
			return false
		Globals.spawnpoint = spawnpoint
		Globals.current_level = to_scene
		if get_tree().change_scene_to_file(to_scene) != OK:
			push_error("Error changing scene")
	pass
