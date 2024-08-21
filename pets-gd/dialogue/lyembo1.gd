extends DialogueScript

func _start() -> void:
    await dbox().say_as("[LYEMBO]", [
        "Hey...?",
        "I don't know what to say.",
        "My name is L'yembo. Maybe we'll meet again someday.",
    ])

    var npcb = get_parent().get_parent()
    npcb.move_to_relative(-200.0, 200.0)
    await npcb.motion_done

    await dbox().say_as("[LYEMBO]", [
        "Motion done.",
    ])
