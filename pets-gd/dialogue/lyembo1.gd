extends DialogueScript

func _start() -> void:
    await dbox().say_as("[LYEMBO]", [
        "Hey...?",
        "I don't know what to say.",
        "My name is L'yembo. Maybe we'll meet again someday.",
    ])
