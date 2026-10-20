use defs::{Element, PointId};

#[derive(Default, Debug, Clone)]
pub struct PostingList {
    entries: Vec<(PointId, Element)>,
}

impl PostingList {
    pub fn insert(&mut self, point_id: PointId, weight: Element) {
        if let Err(pos) = self.position(point_id) {
            self.entries.insert(pos, (point_id, weight));
        }
    }

    pub fn remove(&mut self, point_id: PointId) -> bool {
        match self.position(point_id) {
            Ok(pos) => {
                self.entries.remove(pos);
                true
            }
            Err(_) => false,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (PointId, Element)> + '_ {
        self.entries.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn position(&self, point_id: PointId) -> Result<usize, usize> {
        self.entries.binary_search_by_key(&point_id, |&(id, _)| id)
    }
}
