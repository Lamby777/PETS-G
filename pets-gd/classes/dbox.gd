extends DialogBox

# say a bunch of messages, with choices appearing on the last one
# returns the selected choice's index and value.
#
# if dchoices is a dict, it will use keys for display and values for return
# NOTE: order might be an issue... ^^^
func say_as_with_choices(speaker, msgs, dchoices, do_open = true, do_close = true):
    assert(typeof(dchoices) in [TYPE_ARRAY, TYPE_DICTIONARY], "`typeof(dchoices)` must be an array or dict")

    if do_open:
        open()

    if speaker:
        set_speaker(speaker)

    var index
    for msg_i in msgs.size():
        var msg = msgs[msg_i]

        set_message(msg)
        if msg_i == msgs.size() - 1:
            match typeof(dchoices):
                TYPE_ARRAY:
                    self.queued_choices = dchoices.duplicate()
                TYPE_DICTIONARY:
                    self.queued_choices = dchoices.keys()
        do_draw()
        index = await accept

    if do_close:
        close()

    # NOTE: `value` is NOT locale-dependant. it's safe to use as long as you compare
    # against tr keys and not regular strings. The reason you see regular strings in
    # debug menus is because adding localization for that is usually a waste of time.
    var values = dchoices.values() if typeof(dchoices) == TYPE_DICTIONARY else dchoices

    return { "index": index, "value": values[index] if dchoices.size() > 0 else null }

func say_as(speaker, msgs, do_open = true, do_close = true):
    # allow passing a single string without brackets around it
    if typeof(msgs) == TYPE_STRING:
        msgs = [msgs]

    await say_as_with_choices(speaker, msgs, [], do_open, do_close)

func say(msgs):
    for msg in msgs:
        await say_one(msg)

func say_one(msg):
    set_message(msg)
    do_draw()
    await accept
