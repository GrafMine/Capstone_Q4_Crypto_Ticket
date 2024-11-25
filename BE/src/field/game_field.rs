use super::cell::Cell;

pub struct GameField {
    field: Vec<Vec<Cell>>,
}

impl GameField {
    pub fn new(size: usize) -> Self {
        let mut field = vec![];
        for y in 0..size {
            let mut row = vec![];
            for x in 0..size {
                row.push(Cell::new(String::from("default_color"), x, y, size));
            }
            field.push(row);
        }
        GameField { field }
    }

    pub fn get_cell_by_index(&self, index: usize) -> &Cell {
        let size = self.field.len();
        let y = index / size;
        let x = index % size;
        &self.field[y][x]
    }
}
