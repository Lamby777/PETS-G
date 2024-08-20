use godot::builtin::GString;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldSkill {
    /// Element of the shield
    pub affinity: Affinities,

    /// How many hits the shield can take
    pub hits: u8,

    /// Percent of damage that gets through
    /// If zero, it blocks all damage
    pub multiplier: f64,

    /// Whether the shield reflects damage
    pub reflect: bool,

    /// Does the shield cover the whole party?
    pub plural: bool,
}

impl ShieldSkill {
    fn multi_description(multi: f64) -> GString {
        if multi < 0.0 {
            panic!("shield with negative multiplier");
        }

        let key = if multi == 0.0 {
            "SKILL_SHIELD_POTENCY_IMPENETRABLE"
        } else if multi <= 0.3 {
            "SKILL_SHIELD_POTENCY_STURDY"
        } else if multi <= 0.7 {
            "SKILL_SHIELD_POTENCY_FAIR"
        } else if multi <= 1.0 {
            "SKILL_SHIELD_POTENCY_WEAK"
        } else {
            // if your shield powers have been weakened past 1.0...
            "SKILL_SHIELD_POTENCY_NULLIFIED"
        };

        tr(key)
    }

    fn hits_to_str(hits: u8) -> GString {
        tr(match hits {
            0 => "SKILL_SHIELD_HITS_NONE",
            1 => "SKILL_SHIELD_HITS_ONE",
            2..=3 => "SKILL_SHIELD_HITS_COUPLE",
            4..=6 => "SKILL_SHIELD_HITS_SEVERAL",
            7..=10 => "SKILL_SHIELD_HITS_MANY",
            11.. => "SKILL_SHIELD_HITS_WHILE",
            // _ => unreachable!("shield that can withstand over 15 hits"),
        })
    }

    /// "Wide" or "Narrow"
    fn shield_width_str(&self) -> GString {
        tr(match self.plural {
            true => "SKILL_SHIELD_WIDTH_WIDE",
            false => "SKILL_SHIELD_WIDTH_NARROW",
        })
    }

    /// "Shield" or "Mirror"
    fn shield_type_str(&self) -> GString {
        tr(match self.reflect {
            true => "SKILL_SHIELD_NAME_MIRROR",
            false => "SKILL_SHIELD_NAME_SHIELD",
        })
    }
}

#[typetag::serde]
impl Skill for ShieldSkill {
    fn name(&self) -> String {
        let name = self.shield_type_str();
        let width = self.shield_width_str();
        let adjective = describe_shield_adjective(&self.affinity);

        tr_replace! {
            "SKILL_SHIELD_NAME";
            adjective, width, name,
        }
    }

    fn description(&self) -> String {
        let name = self.shield_type_str();
        let affinity = describe_damage_blocked(&self.affinity);
        let potency = ShieldSkill::multi_description(self.multiplier);
        let width = self.shield_width_str();

        let reflect_action = match self.reflect {
            true => tr!("SKILL_SHIELD_REFLECTIVITY_TRUE"),
            false => tr!("SKILL_SHIELD_REFLECTIVITY_FALSE"),
        };

        let part1 = tr_replace! {
            "SKILL_SHIELD_DESC";
            width, potency, name, reflect_action, affinity,
        };

        let hits_str = ShieldSkill::hits_to_str(self.hits);
        tr_replace! {
            "SKILL_SHIELD_COMBINE_PARTS";
            part1, hits_str
        }
    }

    fn base_cost(&self) -> IntegralStat {
        todo!()
    }

    fn cast(
        &self,
        _caster: Rc<RefCell<Battler>>,
        _target: Rc<RefCell<Battler>>,
        _allies: Vec<Rc<RefCell<Battler>>>,
        _enemies: Vec<Rc<RefCell<Battler>>>,
    ) {
        todo!()
    }
}

pub fn describe_shield_adjective(aff: &Affinities) -> GString {
    if is_physical_shield(aff) {
        return tr("SKILL_SHIELD_PHYSICAL_ADJ");
    }

    if is_magical_shield(aff) {
        return tr("SKILL_SHIELD_MAGICAL_ADJ");
    }

    if is_unique_shield(aff) {
        return tr("SKILL_SHIELD_UNIQUE_ADJ");
    }

    tr("SKILL_SHIELD_SPECIALIZED_ADJ")
}

pub fn describe_damage_blocked(aff: &Affinities) -> GString {
    if is_physical_shield(aff) {
        return tr("SKILL_SHIELD_PHYSICAL_DESC");
    }

    if is_magical_shield(aff) {
        return tr("SKILL_SHIELD_MAGICAL_DESC");
    }

    if is_unique_shield(aff) {
        return tr("SKILL_SHIELD_UNIQUE_DESC");
    }

    tr("SKILL_SHIELD_SPECIALIZED_DESC")
}

fn is_physical_shield(aff: &Affinities) -> bool {
    aff.only_has_all_types(&Element::list_physical())
}

fn is_magical_shield(aff: &Affinities) -> bool {
    aff.only_has_all_types(&Element::list_magical())
}

fn is_unique_shield(aff: &Affinities) -> bool {
    aff.only_has_all_types(&Element::list_magical_and_unique())
}
