use crate::battle::skills::{
    AttackSkill, Element, ShieldSkill, SkillFamily as _,
};
use crate::prelude::*;
use godot::engine::{Sprite2D, Texture2D};
use godot::prelude::*;

fn end_interaction() {
    DialogBox::singleton().bind_mut().end_interaction();
}

fn give_item(item: Item) {
    let inv = si().bind_mut().save.inventory.clone();
    inv.borrow_mut().push(item);
}

impl GodotAutoload for ScriptExecutor {
    const AUTOLOAD_NAME: &'static str = "ScriptExecutor";
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ScriptExecutor {
    base: Base<Node>,
}

#[godot_api]
impl ScriptExecutor {
    #[func]
    fn set_ethan_bed_color(color: String) {
        let texture = load::<Texture2D>(format!(
            "res://assets/textures/builds/furniture/beds/bed_{color}.png"
        ));

        si().bind_mut().save.bed_color = color;

        let mut bed =
            World::room().get_node_as::<Sprite2D>("%EthanBed/Sprite2D");
        bed.callv("set_texture".into(), varray![texture]);

        end_interaction();
        start_ix("Intro #4 >> Bed Color Picked");
    }

    #[func]
    fn debug_skill() {
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
    }

    #[func]
    fn debug_battle() {
        end_interaction();

        World::start_battle(&EnemyID::A_NONNY_MOUSE);
    }

    #[func]
    fn debug_item(item_id: String, quantity: u32) {
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
    }
}
