extends Tactics

@export var bullet: PackedScene
var ctr = 0

func _start():
    print("Start")
    super()
    timer.one_shot = false
    timer.wait_time = 0.5
    timer.start()

func _attack():
    print("Attack" + str(ctr))
    ctr += 1
