class_name BattleStrategy
extends Node

@onready var timer = Timer.new()

func _attack():
    pass

func _ready():
    timer.timeout.connect(_attack)
    timer.start()
