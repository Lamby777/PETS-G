extends DialogueScript

func _start() -> void:
    var picked = await dbox().say_as_with_choices("[RODRICK]", [
        "DG_RODRICK1_SOYOURE",
        "DG_RODRICK1_AREYOUSMART",
    ], [
        "DG_RODRICK1_NOPE",
        "DG_RODRICK1_DEFNOT",
    ])
    
    var msg = ["DG_RODRICK1_DIDNT", "DG_RODRICK1_DEFDIDNT"][picked["index"]]
    await dbox().say_as("[RODRICK]", [msg, "DG_RODRICK1_COMEBACK"])
