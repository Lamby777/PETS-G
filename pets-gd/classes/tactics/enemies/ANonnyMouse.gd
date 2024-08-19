extends Tactics

@export var bullet_scene: PackedScene

func _start():
    super()
    timer.one_shot = false
    timer.wait_time = 0.6
    timer.start()

func _attack():
    var viewport_size = get_viewport().get_visible_rect().size

    for n in range(2):
        var bullet = bullet_scene.instantiate()
        bullet.icon = %BattleIcon
        bullet.position.x = n * viewport_size.x
        bullet.position.y = randi() % int(viewport_size.y)
        bullet.speed = 300 + (randi() % 200)
        %Board.add_child(bullet)
