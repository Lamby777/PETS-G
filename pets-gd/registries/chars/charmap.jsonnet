local BaseBattler = import '../battler.libsonnet';

// this jsonnet file is for the char registry, so ONLY init data!!!!!!

local Base = {
  init_battler: BaseBattler,
};


// the characters that just have the default base stats because im lazy or whatever
local chars_with_default_stats = {
  [x]: Base
  for x in [
    'Devon',
    'Porky',
    'Fuzzy',
    'Terra',
    'Siva',
    'Dylan',
    'Mira',
    'Leo',
    'Lyembo',
    'Quolo',
    'Clay',
    'Juniper',
    'MrTulivae',
    'MsTulivae',
  ]
};

// output the defaults + some with exceptions
chars_with_default_stats {
  Ethan: Base,
}
