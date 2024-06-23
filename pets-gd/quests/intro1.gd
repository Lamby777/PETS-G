extends Quest

@onready var zone1 = $"../../YSort/Room/HouseEntrance"
@onready var dbox = DialogBox.singleton()

func on_zone1_enter(_body):
    zone1.disconnect("body_entered", on_zone1_enter)
    
    # TODO make a signal for when a transition is over
    await PlayerCB.singleton().teleported

    dbox.start_ix("Intro #2")
    
    # connect some other zone's `body_entered` signal

func _ready():
    dbox.start_ix("Intro #1")
    zone1.connect("body_entered", on_zone1_enter)
    
func _process(delta):
    pass
