extends Control

const MAX_PIECES = 4

@onready var rtl = $PanelContainer/VBoxContainer/RichTextLabel
var pieces = []

# Called when the node enters the scene tree for the first time.
func _ready():
    pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
    pass

func push(line: String):
    pieces.append(line)
    if len(pieces) > MAX_PIECES:
        pieces.pop_front()
    
    var text = "\n".join(pieces)
    rtl.set_text(text)
