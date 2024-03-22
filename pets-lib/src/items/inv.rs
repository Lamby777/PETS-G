use super::*;

pub trait ItemList {
    /// Every item that can be equipped
    fn equipment(&self) -> impl Iterator<Item = &Item>;
    fn offsets(&self) -> impl Iterator<Item = &InherentStats>;
}

impl ItemList for &[Item] {
    fn equipment(&self) -> impl Iterator<Item = &Item> {
        self.iter().filter(|i| i.is_equipment())
    }

    fn offsets(&self) -> impl Iterator<Item = &InherentStats> {
        use ItemCat::*;

        self.iter().filter_map(|i| match &i.category {
            Equipment { offsets, .. } => Some(offsets),
            _ => None,
        })
    }
}
