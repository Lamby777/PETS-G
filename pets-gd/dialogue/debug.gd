extends DialogueScript

func _start() -> void:
    var choice
    choice = await dbox().say_as_with_choices(
        "[CASCADE]", [
            "Hey, what's up?",
        ], [
            "Battle",
            "Item",
            "Party",
            "Pauses",
            "Bye",
        ]
    )

    match choice["value"]:
        "Battle": 
            World.start_battle("A_NONNY_MOUSE")

        "Item":
            choice = await dbox().say_as_with_choices(
                "[CASCADE]", [
                    "What do you need?",
                ], [
                    "Rusty x1",
                    "Rusty x5",
                ]
            )

            match choice["index"]:
                0:
                    DialogueScriptBase.debug_item("trusty_rusty_pistol", 1)
                1:
                    DialogueScriptBase.debug_item("trusty_rusty_pistol", 5)

            await dbox().say_as("[CASCADE]", [
                "Here you go!"
            ])
        
        "Party":
            var picked = await dbox().say_as_with_choices(
                "[CASCADE]", [
                    "Who do you want to add?",
                ], [
                    "ETHAN",
                    "SIVA",
                    "TERRA",
                    "MIRA",
                    "LYEMBO",
                    "QUOLO",
                    "LEO",
                    "DYLAN"
                ]
            )

            pcb().push_pchar_gd(picked["value"])

            await dbox().say_as("[CASCADE]", [
                "Welcome to the team" + 
                (", ... me!" if picked["value"] == "MIRA" else "!")
            ])
               

        "Pauses":
            await dbox().say_as("Isaac", [
"""Flint...​​​​​​​​​​
Where did you last see Hinawa?"""
            ])
            await dbox().say_as("[CASCADE]", [
"""This next part will print v​e​r​r​r​r​r​y
s​​​l​​​o​​​o​​​o​​​o​​​o​​​w​​​l​​​y."""
            ])

        "Bye":
            await dbox().say_as("[CASCADE]", [
                "See ya!"
            ])
