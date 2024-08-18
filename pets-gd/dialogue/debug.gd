extends DialogueScript

func _start() -> void:
    print("Opening debug menu!")
    
    var choice
    choice = await dbox().say_as_with_choices(
        "[CASCADE]", [
            "Hey...?",
        ], [
            "Battle",
            "Item",
            "Party",
            "Page 2",
            "Nah",
        ]
    )

    match choice:
        0: # Battle
            World.singleton().start_battle("A_NONNY_MOUSE")

        1: # Item
            choice = await dbox().say_as_with_choices(
                "[CASCADE]", [
                    "What do you need?",
                ], [
                    "Rusty x1",
                    "Rusty x5",
                ]
            )

            match choice:
                0:
                    DialogueScriptBase.debug_item("trusty_rusty_pistol", 1)
                1:
                    DialogueScriptBase.debug_item("trusty_rusty_pistol", 5)

            await dbox().say_as("[CASCADE]", [
                "Here you go!"
            ])
        
        2: # Party
            var choices = [
                "ETHAN",
                "SIVA",
                "TERRA",
                "MIRA",
            ]

            choice = await dbox().say_as_with_choices(
                "[CASCADE]", [
                    "Who do you want to add?",
                ], 
                choices.duplicate()
            )

            var chosen_char = choices[choice]
            PlayerCB.singleton().load_pchar_scene_gd(chosen_char)

            await dbox().say_as("[CASCADE]", [
                "Welcome to the team!"
            ])
               

        3: # Page 2
            choice = await dbox().say_as_with_choices(
                "[CASCADE]", [
                    "Okay, how about these?",
                ], [
                    "Pauses",
                    "Nah",
            ])

            match choice:
                0:
                    await dbox().say_as("Isaac", [
"""Flint...​​​​​​​​​​
Where did you last see Hinawa?"""
                    ])
                    await dbox().say_as("[CASCADE]", [
"""This next part will print v​e​r​r​r​r​r​y
s​​​l​​​o​​​o​​​o​​​o​​​o​​​w​​​l​​​y."""
                    ])
