use super::*;
use crate::consts::battle::ACCESSORY_SLOTS;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Equipment {
    head: Option<String>,
    body: Option<String>,
    weapon: Option<String>,
    accessories: [Option<String>; ACCESSORY_SLOTS],
}

impl Equipment {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.head
            .iter()
            .chain(self.body.iter())
            .chain(self.weapon.iter())
            .chain(self.accessories.iter().filter_map(|s| s.as_ref()))
            .map(|s| s.as_str())
    }

    pub fn offsets(&self) -> LeveledStats {
        self.iter()
            .fold(LeveledStats::zero_all_capable(), |acc, item| {
                let ItemCat::Equipment { ref offsets, .. } =
                    Item::from_registry(item).category
                else {
                    panic!("item {item} not equippable")
                };

                acc + offsets.clone()
            })
    }
}

/// The player's inventory. Keys are item IDs, values are the count of that item.
///
/// # Invariants
/// - All item counts are non-negative
/// - Items with a count of 0 are not present in the map
#[derive(Serialize, Deserialize)]
pub struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get() -> Rc<RefCell<Self>> {
        si().bind().save.inventory.clone()
    }

    pub fn give_item(&mut self, id: impl Into<String>, quantity: i32) {
        let key = id.into();

        let count = self.items.entry(key.clone()).or_insert(0);
        *count = (*count as i32).saturating_add(quantity) as u32;

        if *count == 0 {
            self.items.remove(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn _get_item_count(&self, id: &str) -> u32 {
        self.items.get(id).cloned().unwrap_or(0)
    }

    fn sorted_items(&self) -> Vec<(&String, &u32)> {
        let mut items = self.items.iter().collect::<Vec<_>>();
        items.sort_by_key(|(_, v)| **v);
        items
    }

    pub fn get_at_index(&self, index: usize) -> Option<(&String, &u32)> {
        self.sorted_items().get(index).copied()
    }
}
