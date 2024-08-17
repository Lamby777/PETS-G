extends DialogBox

func say(msgs):
    for msg in msgs:
        set_message(msg)
        do_draw()
        await accept
