pub struct IndexGrid {
    pub entity_ids: Vec<u32>,
}

impl IndexGrid {
    pub fn new() -> Self {
        IndexGrid {
            entity_ids: Vec::new(),
        }
    }
    pub fn insert(&mut self, ids: u32) {
        self.entity_ids.push(ids);
    }
    pub fn get_entities(&self) -> &[u32] {
        &self.entity_ids
    }
}

pub struct PosString {
    pub value: String,
}
use glam::IVec2;
impl From<IVec2> for PosString {
    fn from(value: IVec2) -> Self {
        PosString {
            value: String::from_iter([value.x.to_string(), ",".to_string(), value.y.to_string()]),
        }
    }
}
