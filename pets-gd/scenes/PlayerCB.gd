extends CharacterBody2D


const SPEED = 80.0

func _physics_process(delta):
	var input_direction = Input.get_vector("left", "right", "up", "down")
	velocity = input_direction * SPEED
	
	move_and_slide()
