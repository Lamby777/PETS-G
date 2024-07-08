@tool
class_name StatBar
extends ProgressBar

@onready var bar_label = $Label
@onready var value_label = $Value

@export var bar_label_text: String:
    get:
        return bar_label.text
    set(val):
        if bar_label:
            bar_label.text = val

@export var bar_value: float:
    get:
        return value
    set(val):
        value = val
        if value_label:
            value_label.text = "[right]" + str(val)
