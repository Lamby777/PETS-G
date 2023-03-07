extends CharacterBody2D

const ACCELERATION	:= 3000
const FRICTION		:= 2500
const MAX_SPEED		:= 320

@onready var anim_player = $AnimationPlayer
@onready var anim_tree = $AnimationTree
@onready var anim_state = anim_tree.get("parameters/playback")

func _physics_process(delta):
	var input_vector := Input.get_vector("left", "right", "up", "down").normalized()
	var moving := input_vector != Vector2.ZERO
	
	if moving:
		anim_tree.set("parameters/Idle/blend_position", input_vector)
		anim_tree.set("parameters/Run/blend_position", input_vector)
		anim_state.travel("Run")
		
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
	else:
		anim_state.travel("Idle")
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
