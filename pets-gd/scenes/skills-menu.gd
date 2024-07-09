extends VBoxContainer

@export var choice_agent: ChoiceAgent
@export var battle_engine: BattleEngine
@export var choicelabel_scene: PackedScene

func on_skill_picked(control):
    battle_engine.cast_skill(control.skill_id)
    
func on_skill_hover(control):
    pass

func _ready():
    self.choice_agent.selection_confirmed.connect(on_skill_picked)
