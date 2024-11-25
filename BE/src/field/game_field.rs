use super::{cell::Cell, direction::Direction};

pub struct GameField {
    field: Vec<Vec<Cell>>,
}

impl GameField {
    fn new(size: usize) -> Self {
        let colors = vec!["red", "green", "blue", "yellow"];
        let mut field = vec![];
        for y in 0..size {
            let mut row = vec![];
            for x in 0..size {
                let color = colors[(y * size + x) % colors.len()].to_string();
                row.push(Cell::new(color, x, y, size));
            }
            field.push(row);
        }
        GameField { field }
    }

    fn get_cell_by_index(&self, index: usize) -> &Cell {
        let size = self.field.len();
        let y = index / size;
        let x = index % size;
        &self.field[y][x]
    }

    fn get_moving_cells(&self) -> Vec<&Cell> {
        self.field
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.move_direction.is_some())
            .collect()
    }

    fn apply_movement(&mut self, path: Vec<usize>, direction: Direction) {
        for i in (1..path.len()).rev() {
            let current_index = path[i];
            let next_index = path[i - 1];
    
            let next_color = {
                let next_cell = self.get_cell_by_index(next_index);
                next_cell.current_color.clone() // Clone ensures we have a copy
            };
    
            let current_cell = self.get_cell_by_index_mut(current_index);
    
            if current_cell.next_color == Some(next_color.clone()) {
                panic!("Invalid movement");
            }
    
            current_cell.next_color = Some(next_color);
            current_cell.move_direction = Some(direction);
        }
    }

    fn get_cell_by_index_mut(&mut self, index: usize) -> &mut Cell {
        let size = self.field.len();
        let y = index / size;
        let x = index % size;
        &mut self.field[y][x]
    }
    
}
