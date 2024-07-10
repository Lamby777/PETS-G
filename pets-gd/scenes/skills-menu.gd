extends VBoxContainer

@export var choice_agent: ChoiceAgent
@export var battle_engine: BattleEngine
@export var choicelabel_scene: PackedScene
@export var description_label: RichTextLabel

func on_skill_picked(control):
    battle_engine.cast_skill(control.skill_id)
    
func on_skill_hover(control):
    description_label.text = battle_engine.describe_skill(control.skill_id)

func _ready():
    self.choice_agent.selection_confirmed.connect(on_skill_picked)
    self.choice_agent.selection_focused.connect(on_skill_hover)
