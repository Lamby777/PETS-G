extends DialogueScript

func _start() -> void:
    # TODO: if player has not yet gone to school to earn the scrapbook
    # pretend like she's busy and doesn't notice you walking by

    await dbox().say_as("DG_SPK_UNKNOWN", "DG_BECKY_INTRO_HI")

    var picked = await dbox().say_as_with_choices("DG_SPK_BECKY", [
        "DG_BECKY_INTRO_HOSTILES",
        "DG_BECKY_INTRO_DONT_WORRY",
        "DG_BECKY_INTRO_STUDY",
        "DG_BECKY_INTRO_HELP",
        "DG_BECKY_INTRO_LOGGING",
        "DG_BECKY_INTRO_OTHER_ENEMIES",
        "DG_BECKY_INTRO_MILESTONES",
        "DG_BECKY_INTRO_OFFER",
    ], [
        "DG_BECKY_INTRO_ACCEPT",
        "DG_BECKY_INTRO_DECLINE",
    ])

    var accepted_book = picked["index"] == 0

    if not accepted_book:
        await dbox().say_as("DG_SPK_BECKY", "DG_BECKY_INTRO_OH_WELL")
        return

    await dbox().say_as("DG_SPK_BECKY", "DG_BECKY_INTRO_GREAT_HERES_YOUR")

    # TODO: the user accepted the logbook, so grant access to it and pop up
    # a message saying that Becky's log has been added to the scrapbook
