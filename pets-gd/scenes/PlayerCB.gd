extends CharacterBody2D

"""
This scene contains the "player" aka the invisible
entity that is moved around with WASD. It also contains
party members as scenes, and this script does stuff like
running animations on those nodes too.
"""

# Movement physics stuff
const ACCELERATION	:= 3000
const FRICTION		:= 2500
const MAX_SPEED		:= 320

# Distance between party members
const PERSONAL_SPACE := 300

@onready var test = $Character

var pastPositions := LimitedQueue.new(2000)
@onready var party: Array[PChar] = [
	test
]

func _physics_process(delta):
	var input_vector := Input.get_vector("left", "right", "up", "down").normalized()
	var moving := input_vector != Vector2.ZERO
	
	if moving:
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
	else:
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
	
	if (pastPositions.get_len() == 0) or (pastPositions.get_at(0) != position):
		pastPositions.push_front(global_position)

func _process(_delta):
	move_chars()

func move_chars():
	for i in party.size():
		var ch := party[i]
		ch.global_position = pastPositions.get_at(i * PERSONAL_SPACE)
