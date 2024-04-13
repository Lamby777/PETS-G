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
