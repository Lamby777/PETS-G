extends BattleStrategy

var ctr = 0

func _ready():
    timer.one_shot = false
    timer.wait_time = 0.1
    super()

func _attack():
    print("Attack" + str(ctr))
    ctr += 1
