extends CharacterBody2D


const SPEED = 80.0

func _physics_process(delta):
	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	######### ^^^^^^^^ IMPORTANT, do this later
	var dirs_xy = [
		Input.get_axis("ui_left", "ui_right"),
		Input.get_axis("ui_up", "ui_down")
	]
	
	for i in range(2):
		var axis = ["x", "y"][i]
		
		if dirs_xy[i]:
			velocity[axis] = dirs_xy[i] * SPEED
		else:
			velocity[axis] = move_toward(velocity[axis], 0, SPEED)
	
	move_and_slide()
