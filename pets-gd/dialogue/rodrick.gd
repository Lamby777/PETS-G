extends DialogueScript

func _start() -> void:
    dbox().open()
    dbox().set_speaker("[RODRICK]")
    dbox().say("DG_RODRICK1_SOYOURE")
    await dbox().accept
    dbox().close()
