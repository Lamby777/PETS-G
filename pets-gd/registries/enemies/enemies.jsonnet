local BaseBattler = import '../battler.libsonnet';

local BaseChar = {
  level: 1,
  battler: BaseBattler,
};

local NewCharacter(id, display_name=null, overrides={}) =
  BaseChar {
    id: id,
    display_name: if display_name == null then id else display_name,
  } + overrides;

[
  NewCharacter('Devon'),
  NewCharacter('Porky'),
  NewCharacter('Terra'),
  NewCharacter('Siva'),
  NewCharacter('Dylan'),
  NewCharacter('Mira'),
  NewCharacter('Leo'),
  NewCharacter('Fuzzy'),
  NewCharacter('Quolo'),
  NewCharacter('Clay'),
  NewCharacter('Juniper'),

  NewCharacter('Lyembo', display_name="L'yembo"),
  NewCharacter('MrTulivae', display_name='Mr. Tulivae'),
  NewCharacter('MsTulivae', display_name='Ms. Tulivae'),

  NewCharacter('Ethan', overrides={
    battler+: {
      equipment+: {
        weapon: 'cool_stick',
      },
    },
  }),
]
