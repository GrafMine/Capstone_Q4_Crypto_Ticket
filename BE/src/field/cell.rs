#[derive(Debug, Clone)]
pub struct Cell {
    pub id: usize,
}

impl Cell {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
