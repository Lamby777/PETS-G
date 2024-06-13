extends Quest

@onready var zone1 = $"../../YSort/Room/TeleporterDemo/TPRightOut"

func on_zone1_enter(_body):
	if self.phase != 0: return
	
	self.phase = 1
	print("Teleport done!")

func _ready():
	zone1.connect("body_entered", on_zone1_enter)
	
func _process(delta):
	pass
