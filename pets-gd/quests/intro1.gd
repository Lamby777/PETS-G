extends Quest

@onready var dbox = DialogBox.singleton()
@onready var pcb = PlayerCB.singleton()

func room_id():
    return $"../../YSort/Room".room_id

func _ready():
    dbox.start_ix("Intro #1")
    pcb.teleported.connect(on_teleported)

    on_world_tp()

func on_teleported(target):
    # It needs to know which room you're teleporting inside, because
    # the rooms have different teleporters that may or may not exist
    if room_id() == "CV_Outdoors":
        on_world_tp()
    elif room_id() == "CV_Houses":
        on_house_tp(target)

func on_world_tp():
    await pcb.teleported

    $"../../YSort/Room/HouseEntrance".body_entered.connect(on_world_tp)

    if phase < 1:
        self.phase = 1
        dbox.start_ix("Intro #2")


func on_house_tp(target):
    await pcb.teleported

    $"../../YSort/Room/EthanBedroomEntrance".body_entered.connect(on_house_tp)

    if phase < 2:
        self.phase = 2
        dbox.start_ix("Intro #2")

    if target.name == "EthanBedroomEntrance" and phase < 3:
        self.phase = 3
        dbox.start_ix("Intro #3")
