extends CharacterBody2D

const ACCELERATION	:= 3000
const FRICTION		:= 2500
const MAX_SPEED		:= 320
const DIRECTION_STRINGS := [
	"R", "DR", "D", "DL", "L", "UL", "U", "UR",
]

@onready var anim_player = $AnimationPlayer
var last_direction := "D"

func _physics_process(delta):
	var input_vector := Input.get_vector("left", "right", "up", "down").normalized()
	var moving := input_vector != Vector2.ZERO
	
	if moving:
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
		last_direction = vel_to_dirS(input_vector)
	else:
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
	
	anim_player.play(("Run" if moving else "Idle") + last_direction)

func vel_to_dirS(vel: Vector2):
	var angle := vel.angle()
	if angle < 0: angle += 2 * PI
	
	return DIRECTION_STRINGS[round(angle / PI * 4)]
