use std::cell::LazyCell;

use crate::battle::skills::{
    AttackSkill, Element, ShieldSkill, SkillFamily as _,
};
use crate::prelude::*;
use godot::engine::{StaticBody2D, Texture2D};
use godot::prelude::*;

fn end_interaction() {
    DialogBox::singleton().bind_mut().end_interaction();
}

fn give_item(item: Item) {
    let inv = si().bind_mut().save.inventory.clone();
    inv.borrow_mut().push(item);
}

macro_rules! add_callables {
    ($table:expr; { $($fn_name:ident),* $(,)? }) => {
        $(
            let name = stringify!($fn_name);
            let callable = Callable::from_fn(name, $fn_name);
            $table.insert(name.to_string(), callable);
        )*
    };
}

/// Call a function registered in the global function table.
/// To pass args, use `callv_global`.
pub fn call_global(id: &str) -> GReturn {
    callv_global(id, VariantArray::new())
}

/// Call a function registered in the global function table.
pub fn callv_global(func_id: &str, args: VariantArray) -> GReturn {
    let funcs = FUNCTIONS;
    let func = funcs.get(func_id);
    let func = unwrap_fmt!(func, "no function named {}", func_id);

    let res = func.callv(args);
    Ok(res)
}

const FUNCTIONS: LazyCell<FnTable> = LazyCell::new(|| {
    let mut table = HashMap::new();
    add_callables!(table; {
        debug_battle,
        debug_item,
        debug_skill,
        set_ethan_bed_color,
    });

    table
});

fn set_ethan_bed_color(args: GArgs) -> GReturn {
    let color = args[0].to::<String>();
    let texture = load::<Texture2D>(format!(
        "res://assets/textures/builds/furniture/beds/bed_{color}.png"
    ));

    si().bind_mut().save.bed_color = color;

    let mut bed =
        World::room().get_node_as::<StaticBody2D>("%EthanBed/Sprite2D");

    bed.callv("set_texture".into(), varray![texture]);

    Ok(Variant::nil())
}

fn debug_skill(_args: GArgs) -> GReturn {
    end_interaction();

    let skill1 = AttackSkill {
        tr_key: "SKILL_ATTACK_FIRE_DMG_NAME".to_owned(),
        element: Element::Fire,
        power: Some(1),
        plural: true,
        status_effect: None,
    };

    let aff = {
        let mut map = HashMap::new();
        map.insert(Element::Fire, AffinityPower::Weak);
        map.insert(Element::Fuzz, AffinityPower::Strong);
        map.insert(Element::Wind, AffinityPower::Nullify);
        map.insert(Element::Spirit, AffinityPower::Heal);
        map.insert(Element::Ice, AffinityPower::Reflect);
        Affinities::new(map)
    };

    let skill2 = ShieldSkill {
        affinity: aff,
        hits: 1,
        multiplier: 0.2,
        reflect: false,
        plural: false,
    };

    start_ix_replace("Debug Menu >> Skill", &[
        ("{SKILL_NAME1}".to_owned(), skill1.name()),
        ("{SKILL_DESC1}".to_owned(), skill1.description()),
        ("{SKILL_NAME2}".to_owned(), skill2.name()),
        ("{SKILL_DESC2}".to_owned(), skill2.description()),
    ]);

    Ok(Variant::nil())
}

fn debug_battle(_args: GArgs) -> GReturn {
    end_interaction();

    World::start_battle(&EnemyID::A_NONNY_MOUSE);
    Ok(Variant::nil())
}

fn debug_item(args: GArgs) -> GReturn {
    let item_id = args[0].to::<String>();

    let quantity = args
        .get(1)
        .map(|v| v.try_to::<String>().ok())
        .flatten()
        .map(|v| v.parse().ok())
        .flatten()
        .unwrap_or(1);

    // why tf do i have to do this?
    let item = ITEM_REGISTRY
        .get()
        .unwrap()
        .iter()
        .find(|i| i.id == item_id);

    let item = ribbons::unwrap_fmt!(item, "no item with id {}", item_id);

    for _ in 0..quantity {
        give_item(item.clone());
    }

    end_interaction();
    start_ix("Debug Menu >> After Item");

    Ok(Variant::nil())
}
