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

use super::BorderDir;
impl From<IVec2> for PosString {
    fn from(value: IVec2) -> Self {
        PosString {
            value: String::from_iter([value.x.to_string(), ",".to_string(), value.y.to_string()]),
        }
    }
}
impl From<(IVec2, BorderDir)> for PosString {
    fn from(value: (IVec2, BorderDir)) -> Self {
        PosString {
            value: String::from_iter([
                value.0.x.to_string(),
                ",".to_string(),
                value.0.y.to_string(),
                value.1.to_string(),
            ]),
        }
    }
}
