extends DialogueScript

func _start() -> void:
    var choice
    choice = await dbox().say_as_with_choices(
        "DG_SPK_CASCADE", [
            "Hey, what's up?",
        ], [
            "Party",
            "Item",
            # "Battle",
            "Bye",
        ]
    )

    match choice["value"]:
        # "Battle": 
        #     World.start_battle("ANonnyMouse")

        "Item":
            choice = await dbox().say_as_with_choices(
                "DG_SPK_CASCADE", [
                    "What do you need?",
                ], [
                    "Rusty x1",
                    "Stick x1",
                    "Bundle",
                ]
            )

            match choice["index"]:
                0:
                    DialogueScript.debug_item("trusty_rusty_pistol", 1)
                1:
                    DialogueScript.debug_item("cool_stick", 1)
                2:
                    DialogueScript.debug_item("trusty_rusty_pistol", 5)
                    DialogueScript.debug_item("cool_stick", 5)

            await dbox().say_as("DG_SPK_CASCADE", [
                "Here you go!"
            ])
        
        "Party":
            var picked = await dbox().say_as_with_choices(
                "DG_SPK_CASCADE", [
                    "Who do you want to add?",
                ], {
                    "...": "...",
                    "E": "Ethan",
                    "S": "Siva",
                    "T": "Terra",
                    "AC": "Mira",
                    "L": "Lyembo",
                    "Q": "Quolo",
                    "Leo": "Leo",
                    "Dylan": "Dylan",
                    "Poof!": "Poof!",
                }
            )

            match picked["value"]:
                "...":
                    pcb().push_pchar_gd("Siva")
                    pcb().push_pchar_gd("Terra")
                    pcb().push_pchar_gd("Leo")
                    pcb().push_pchar_gd("Lyembo")
                    pcb().push_pchar_gd("Quolo")
                    await dbox().say_as("DG_SPK_CASCADE", "Well, here are a bunch of people.")
                "Poof!":
                    pcb().wipe_party(true)
                    await dbox().say_as("DG_SPK_CASCADE", "Wow, that's so weird! They just disappeared!")
                var picked_value:
                    pcb().push_pchar_gd(picked_value)
                    await dbox().say_as("DG_SPK_CASCADE", "Welcome to the team" +
                        (", ... me!" if picked_value == "Mira" else "!"))
