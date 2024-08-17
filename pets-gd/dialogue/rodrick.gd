extends DialogueScript

func _start() -> void:
    await dbox().say_as_with_choices("[RODRICK]", [
        "DG_RODRICK1_SOYOURE",
        "DG_RODRICK1_AREYOUSMART",
    ], [
        "DG_RODRICK1_NOPE",
        "DG_RODRICK1_DEFNOT",
    ])
    
    # DG_RODRICK1_DIDNT
    # DG_RODRICK1_DEFDIDNT
    #
    # DG_RODRICK1_COMEBACK
