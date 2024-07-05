class_name Bullet
extends Node2D

@export var damage_ratio: float = 1.0
@onready var area = $Area2D

# this is so we can do stuff relating to the icon.
# anyone instantiating a bullet MUST set this immediately!
@export var icon: BattleIcon

func on_hit(_body):
    icon.on_hit(self)

func _ready():
    area.connect("body_entered", on_hit)
