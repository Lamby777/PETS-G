class_name Tactics
extends Node

var timer: Timer

func _attack():
    pass

func _start():
    print("Base start")
    timer = Timer.new()
    timer.name = "MainTimer"
    timer.timeout.connect(_attack)
    add_child(timer)

func _ready():
    _start()
