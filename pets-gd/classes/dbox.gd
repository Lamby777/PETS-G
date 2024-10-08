extends DialogBox

# say a bunch of messages, with choices appearing on the last one
# returns the selected choice's index
func say_as_with_choices(speaker, msgs, dchoices, do_open = true, do_close = true):
    if do_open:
        open()
        
    if speaker:
        set_speaker(speaker)

    var index
    for msg_i in msgs.size():
        var msg = msgs[msg_i]
        
        set_message(msg)
        if msg_i == msgs.size() - 1:
            self.queued_choices = dchoices.duplicate()
        do_draw()           
        index = await accept
        
    if do_close:
        close()
    
    return { "index": index, "value": dchoices[index] if dchoices.size() > 0 else null }
       
func say_as(speaker, msgs, do_open = true, do_close = true):
    await say_as_with_choices(speaker, msgs, [], do_open, do_close)

func say(msgs):
    for msg in msgs:
        await say_one(msg)

func say_one(msg):
    set_message(msg)
    do_draw()
    await accept
