use crate::VectorIndex;

pub struct HnswIndex {

}

impl HnswIndex {
    pub fn new() -> Self {
        Self {}
    }
}

impl VectorIndex for HnswIndex {
    fn insert(&self, vector: &[f32]) -> Result<(), Error> {
        Ok(())
    }

    fn delete(&self, vector: &[f32]) -> Result<(), Error> {
        Ok(())
    }
    
    fn search(&self, vector: &[f32]) -> Result<Vec<f32>, Error> {
        Ok(vec![])
    }

    fn build(&self) -> Result<(), Error> {
        Ok(())
    }
}