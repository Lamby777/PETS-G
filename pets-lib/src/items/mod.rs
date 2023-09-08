pub struct Item {
    categories: Vec<ItemCategory>,
}

pub enum ItemCategory {
    Weapon,
    Armor,
    Key,
    Consumable,
}
