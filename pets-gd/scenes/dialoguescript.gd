class_name DialogueScript
extends DialogueScriptBase

func say(msgs):
    for msg in msgs:
        dbox().set_message(msg)
        dbox().do_draw()
        await dbox().accept
