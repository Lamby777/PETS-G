extends Quest

func room_id():
    return $"../../YSort/Room".room_id

func _ready():
    pcb().move_to_relative(0.0, -200.0)
    await pcb().motion_done
    
    dbox().open()
    dbox().set_speaker("[JUNIPER]")
    await dbox().say(["DG_INTRO1_COMEUNPACK"])
    dbox().close()

    pcb().teleported.connect(on_teleported)

func on_teleported(target):
    # It needs to know which room you're teleporting inside, because
    # the rooms have different teleporters that may or may not exist
    if room_id() == "CV_Houses":
        on_house_tp(target)
    if room_id() == "CV_Outdoors":
        on_outdoors_tp(target)

func on_house_tp(target):
    if phase == 0:
        self.phase = 1
        dbox().open()
        dbox().set_speaker("[JUNIPER]")
        await dbox().say(["DG_INTRO1_OVERHERE"])
        dbox().close()

    if target.name == "EthanBedroomExit" and phase == 1:
        self.phase = 2
        #dbox().start_ix("Intro #3")

func on_outdoors_tp(target):
    if target.name == "EthanHouseEntrance" and phase == 2:
        self.phase = 3
        #dbox().start_ix("Intro #5")
