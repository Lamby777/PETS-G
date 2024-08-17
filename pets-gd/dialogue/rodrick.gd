extends DialogueScript

func _start() -> void:
    dbox().open()
    dbox().set_speaker("[RODRICK]")
    dbox().say("DG_RODRICK1_SOYOURE")
    await dbox().accept
    dbox().say("DG_RODRICK1_AREYOUSMART")
    await dbox().accept

    # > DG_RODRICK1_NOPE
    # DG_RODRICK1_DIDNT
    # > DG_RODRICK1_DEFNOT
    # DG_RODRICK1_DEFDIDNT
    #
    # DG_RODRICK1_COMEBACK

    dbox().close()
