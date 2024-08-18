extends Quest

func room_id():
    return $"../../YSort/Room".room_id

func _ready():
    pcb().move_to_relative(0.0, -200.0)
    await pcb().motion_done
    
    await dbox().say_as("[JUNIPER]", ["DG_INTRO1_COMEUNPACK"])
    
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
        await dbox().say_as("[JUNIPER]", ["DG_INTRO1_OVERHERE"])

    if target.name == "EthanBedroomExit" and phase == 1:
        self.phase = 2
        var choicelist = ["DG_INTRO1_GREAT", "DG_INTRO1_ALRIGHT"] 
        var picked = await dbox().say_as_with_choices(
            "[JUNIPER]",
            ["DG_INTRO1_WDYT"],
            choicelist.duplicate(),
        )

        var picked_bedcolor_i = await dbox().say_as_with_choices(
            "[JUNIPER]",
            [choicelist[picked] + "_MOM", "DG_INTRO1_ITSYOURS"],
            [
                "[color=#BD0000]X[/color]",
                "[color=#FF7200]X[/color]",
                "[color=#FFE300]X[/color]",
                "[color=#00AE06]X[/color]",
                "[color=#2500FF]X[/color]",
                "[color=#7F0087]X[/color]",
                "[color=#E889C4]X[/color]",
            ]
        )

        var bedcolors = ["red", "orange", "yellow", "green", "blue", "purple", "pink"]
        var picked_bedcolor = bedcolors[picked_bedcolor_i]

        DialogueScriptBase.set_ethan_bed_color(picked_bedcolor)

        await dbox().say_as("[JUNIPER]", [
            "DG_INTRO1_LOOKSBETTER",
            "DG_INTRO1_ILLBEDOWNSTAIRS",
            "DG_INTRO1_BRINGFUZZY",
        ])

func on_outdoors_tp(target):
    if target.name == "EthanHouseEntrance" and phase == 2:
        self.phase = 3
        await dbox().say_as("[CLAY]", [ "DG_INTRO1_MEET_NEIGHBORS" ]);
        var pcb = PlayerCB.singleton()
        pcb.move_to_relative(-200.0, 0.0)
        await pcb.motion_done
        await dbox().say_as("[CLAY]", [ "DG_INTRO1_MR_T" ]);
        await dbox().say_as("[MR_TULIVAE]", [ "DG_INTRO1_HITHERE" ]);
        await dbox().say_as("[NARRATOR]", [ "DG_INTRO1_HE_SEEMS_NICE" ]);

