// base template. actual item instances add onto this.
local Equipment = {
    category: {
        Equipment: {
            category: 'Weapon',
            offsets: {
                max_hp: 0,
                max_energy: 0,
                attack: 0,
                defense: 0,
                speed: 0,
                stability: 0,
                delta: 0,
                epsilon: 0,
                lambda: 0,
                max_mana: 0,
            },
            equippable_by: [],
        },
    },
};

{
    cool_stick: Equipment {
        attributes: ['Cheap', 'Melee'],
        category+: {
            Equipment+: {
                offsets+: {
                    attack: 1,
                    speed: 2,
                    stability: 3,
                },
                equippable_by: ['Ethan'],
            },
        },
    },

    trusty_rusty_pistol: Equipment {
        attributes: ['Cheap', 'Ranged'],
        category+: {
            Equipment+: {
                offsets+: {
                    attack: 2,
                    speed: 3,
                    stability: 1,
                },
                equippable_by: ['Terra'],
            },
        },
    },
}
