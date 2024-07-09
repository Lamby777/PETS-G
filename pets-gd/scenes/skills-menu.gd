extends VBoxContainer

@export var choice_agent: ChoiceAgent
@export var battle_engine: BattleEngine

func on_skill_picked(control):
    battle_engine.cast_skill("Caustics A")
    
func on_skill_hover(control):
    pass

func _ready():
    self.choice_agent.selection_confirmed.connect(on_skill_picked)
