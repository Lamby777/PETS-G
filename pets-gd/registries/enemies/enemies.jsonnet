local BaseBattler = import '../battler.libsonnet';

local Base = {
  level: 1,
  battler: BaseBattler,
};

// null display_name means the game uses the default name from translations
local NewCharacter(id, display_name=null, overrides={}) =
  Base {
    id: id,
    display_name: display_name,
  } + overrides;

[
  NewCharacter('ANonnyMouse'),

  NewCharacter('Ethan', overrides={
    battler+: {
      equipment+: {
        weapon: 'cool_stick',
      },
    },
  }),
]
