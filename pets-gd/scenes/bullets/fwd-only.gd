class_name ForwardBullet
extends Bullet

@export var speed = 400.0

func _ready():
    look_at(icon.position)

func _process(delta):
    var move_vec = Vector2(speed, 0).rotated(rotation)
    global_position += move_vec * delta
