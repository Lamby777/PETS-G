class_name Globals
extends Node

#func _input(_event):
    #if Input.is_key_pressed(KEY_0):
        #get_tree().change_scene_to_file("res://scenes/world.tscn")
    #elif Input.is_key_pressed(KEY_MINUS):
        #get_tree().quit()

func _ready():
    discord_rich_presence()

func discord_rich_presence():
    DiscordRPC.app_id = 1115759778644365353
    print("Discord working: " + str(DiscordRPC.get_is_discord_working()))
    DiscordRPC.details = "Epic Gaming"
    DiscordRPC.state = "@ Corr Valley"
    
    DiscordRPC.large_image = "mira-what"
    DiscordRPC.large_image_text = "Epic Gaming, indeed."
    DiscordRPC.small_image = "mira-what"
    DiscordRPC.small_image_text = "Catchin' Zs."

    DiscordRPC.start_timestamp = int(Time.get_unix_time_from_system()) # "02:46 elapsed"
    # discord_sdk.end_timestamp = int(Time.get_unix_time_from_system()) + 3600 # +1 hour in unix time / "01:00 remaining"

    # Always refresh after changing the values!
    DiscordRPC.refresh()
