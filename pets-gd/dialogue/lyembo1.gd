extends DialogueScript

func _start() -> void:
    await dbox().say_as("[LYEMBO]", [
        "Hey...?",
        "I don't know what to say.",
        "My name is L'yembo. Maybe we'll meet again someday.",
    ])

    var lyembo = get_parent().get_parent()
    lyembo.move_to_relative(-100.0, 0.0)
    await lyembo.pchar_motion_done
