class_name Bullet
extends Node2D

@export var damage_ratio: float = 1.0
@onready var area = $Area2D

func on_hit():
    pass

func _ready():
    area.body_entered.connect(on_hit)
