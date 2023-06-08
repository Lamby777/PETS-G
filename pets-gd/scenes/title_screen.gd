extends PanelContainer

@onready var start_sound = $StartSound

func _ready():
	start_sound.play()

func _process(_delta):
	pass
