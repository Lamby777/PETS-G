extends CharacterBody2D

const ACCELERATION	= 6000
const FRICTION		= 5000
const MAX_SPEED		= 400

func _physics_process(delta):
	var input_vector = Input.get_vector("left", "right", "up", "down").normalized()
	
	if input_vector != Vector2.ZERO:
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
	else:
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
