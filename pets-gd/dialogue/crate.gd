extends DialogueScript

func _start() -> void:
    await dbox().say_as("[NARRATOR]", [
        "DG_CHECK_CRATE",
    ])
