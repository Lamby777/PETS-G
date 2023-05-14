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
const PERSONAL_SPACE := 30

@onready var agentE = $AgentE
@onready var agentS = $AgentS

var pastPositions := LimitedQueue.new(2000)
var pastRotations := LimitedQueue.new(2000)
@onready var party: Array[PChar] = [
	agentE,
	agentS,
]

func _physics_process(delta):
	var input_vector := Input.get_vector("left", "right", "up", "down").normalized()
	var moving := input_vector != Vector2.ZERO
	
	if moving:
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
	else:
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
	
	var posUpdated: bool = (
		(pastPositions.get_len() == 0) or
		(pastPositions.get_at(0) != position)
	)
	
	if posUpdated:
		pastPositions.push_front(global_position)
		pastRotations.push_front(input_vector)
	
	move_chars(moving)

func move_chars(moving: bool):
	if pastPositions.get_len() == 0: return
	
	for i in party.size():
		var ch := party[i]
		
		# index of past data limqs
		var nth = i * PERSONAL_SPACE
		
		ch.global_position = pastPositions.get_or_last(nth)
		ch.anim_move(moving, pastRotations.get_or_last(nth))
