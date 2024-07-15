use super::*;

pub trait ItemList {
    /// Every item that can be equipped
    fn offsets(&self) -> impl Iterator<Item = &InherentStats>;
}

impl ItemList for &[Item] {
    fn offsets(&self) -> impl Iterator<Item = &InherentStats> {
        use ItemCat::*;

        self.iter().filter_map(|i| match &i.category {
            Equipment { offsets, .. } => Some(offsets),
            _ => None,
        })
    }
}

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

    pub fn give_item(&mut self, id: String, quantity: u32) {
        let count = self.items.entry(id).or_insert(0);
        *count += quantity;
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_item_count(&self, id: &str) -> u32 {
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
