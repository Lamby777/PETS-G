extends Quest

@onready var dbox = DialogBox.singleton()
@onready var pcb = PlayerCB.singleton()

func room_id():
    return $"../../YSort/Room".room_id

func _ready():
    dbox.start_ix("Intro #1")
    pcb.teleported.connect(on_teleported)

func on_teleported(target):
    # It needs to know which room you're teleporting inside, because
    # the rooms have different teleporters that may or may not exist
    if room_id() == "CV_Houses":
        on_house_tp(target)

func on_house_tp(target):
    if phase < 2:
        self.phase = 2
        dbox.start_ix("Intro #2")

    if target.name == "EthanBedroomExit" and phase < 3:
        self.phase = 3
        dbox.start_ix("Intro #3")
