extends DialogueScript

func _start() -> void:
    dbox().open()
    dbox().set_speaker("[RODRICK]")
    await dbox().say([
        "DG_RODRICK1_SOYOURE",
        "DG_RODRICK1_AREYOUSMART"
    ])
    
    # > DG_RODRICK1_NOPE
    # DG_RODRICK1_DIDNT
    # > DG_RODRICK1_DEFNOT
    # DG_RODRICK1_DEFDIDNT
    #
    # DG_RODRICK1_COMEBACK

    dbox().close()
