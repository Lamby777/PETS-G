class_name Globals
extends Node
    
func _input(_event):
    if Input.is_key_pressed(KEY_0):
        get_tree().change_scene_to_file("res://scenes/world.tscn")
    elif Input.is_key_pressed(KEY_MINUS):
        get_tree().quit()

func _ready():
    discord_rich_presence()

func discord_rich_presence():
    discord_sdk.app_id = 1115759778644365353
    print("Discord working: " + str(discord_sdk.get_is_discord_working()))
    discord_sdk.details = "Epic Gaming"
    discord_sdk.state = "@ Corr Valley"
    
    discord_sdk.large_image = "siva-aurora"
    discord_sdk.large_image_text = "Epic Gaming, indeed."
    discord_sdk.small_image = "siva-sleeping"
    discord_sdk.small_image_text = "Catchin' Zs."

    discord_sdk.start_timestamp = int(Time.get_unix_time_from_system()) # "02:46 elapsed"
    # discord_sdk.end_timestamp = int(Time.get_unix_time_from_system()) + 3600 # +1 hour in unix time / "01:00 remaining"

    # Always refresh after changing the values!
    discord_sdk.refresh()

