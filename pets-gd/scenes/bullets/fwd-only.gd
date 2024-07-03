class_name ForwardBullet
extends Bullet

@export var speed = 1.0

func _process(delta):
    var move_vec = Vector2(speed, 0).rotated(rotation)
    global_position += move_vec * delta
