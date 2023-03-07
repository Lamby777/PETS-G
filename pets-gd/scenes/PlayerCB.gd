extends CharacterBody2D

const ACCELERATION = 12000
const FRICTION = 20000
const MAX_SPEED = 10

var vel = Vector2.ZERO

func _physics_process(delta):
	var input_vector = Input.get_vector("left", "right", "up", "down").normalized()
	
	if input_vector != Vector2.ZERO:
		vel += (
			input_vector *
			ACCELERATION *
			delta
		).clamp(Vector2(-MAX_SPEED, -MAX_SPEED), Vector2(MAX_SPEED, MAX_SPEED))
	else:
		vel = vel.move_toward(Vector2.ZERO, FRICTION * delta)
	
	velocity = vel
	move_and_slide()
