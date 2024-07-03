extends Tactics

@export var bullet: PackedScene

func _start():
    print("Start")
    super()
    timer.one_shot = false
    timer.wait_time = 1
    timer.start()

func _attack():
    print("Attack!")
    var bullet = bullet.instantiate()
    bullet.icon = %BattleIcon
    %Board.add_child(bullet)
