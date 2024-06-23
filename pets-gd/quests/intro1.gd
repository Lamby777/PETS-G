extends Quest

@onready var zone1 = $"../../YSort/Room/TeleporterDemo/TPRightOut"
@onready var dbox = DialogBox.singleton()

func on_zone1_enter(_body):
    print("Teleport done!")
    dbox.start_ix("Intro #2")

    zone1.disconnect("body_entered", on_zone1_enter)
    # connect some other zone's `body_entered` signal

func _ready():
    dbox.start_ix("Intro #1")
    zone1.connect("body_entered", on_zone1_enter)
    
func _process(delta):
    pass
