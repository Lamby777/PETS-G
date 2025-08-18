local BaseChar = {
    level: 1,
    battler: {
        battle_stats: {
            hp: 30,
            mana: null,
            energy: 1,
            buffs: [],
        },
        status_effects: [],
        buffs_list: [],
        inherent_stats: {
            max_hp: 30,
            max_energy: 1,
            attack: 1,
            defense: 1,
            speed: 1,
            stability: 1,
            delta: 1,
            epsilon: 1,
            lambda: null,
            max_mana: null,
        },
        equipment: {
            head: null,
            body: null,
            weapon: null,
            accessories: [null, null, null],
        },
    },
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
