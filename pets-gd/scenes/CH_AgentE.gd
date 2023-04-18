extends Node2D

@onready var anim_tree = $AnimationTree
@onready var anim_state = anim_tree.get("parameters/playback")

func _physics_process(delta):
	# If moving, do this stuff:
#	anim_tree.set("parameters/Idle/blend_position", input_vector)
#	anim_tree.set("parameters/Run/blend_position", input_vector)
#	anim_state.travel("Run")

	# Else
#	anim_state.travel("Idle")
