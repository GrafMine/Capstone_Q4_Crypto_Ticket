use super::{cell::Cell, direction::Direction};

pub struct GameFieldService {
    rows: usize,
    cols: usize,
    field: Vec<Cell>,
}

impl GameFieldService {
    pub fn new(size: usize) -> Self {
        let mut field = vec![];

        for id in 0..(size * size) {
            field.push(Cell::new(id));
        }

        Self { rows:size, cols:size, field }
    }

    pub fn move_cell(&mut self, start_id: usize, direction: Direction, diff: usize) {
        let mut current_id = start_id;
        let mut steps = 0;

        while steps < diff {
            if let Some(next_id) = self.calculate_target(current_id, direction) {
                println!(
                    "Step {}: move cell from index {} to index {} in direction {:?}",
                    steps + 1,
                    current_id,
                    next_id,
                    direction
                );

                let current_cell = self.get_cell_by_index(current_id).clone();
                self.field[next_id] = current_cell;

                current_id = next_id;
                steps += 1;
            } else {
                println!(
                    "The cell {} cannot move to {:?} (reached end of field).",
                    current_id, direction
                );
                break;
            }
        }
    }

    pub fn print_field(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = i * self.cols + j;
                print!("{} ", self.field[index].id);
            }
            println!(); // Новий рядок після кожного рядка
        }
    }

    fn get_cell_by_index(&self, index: usize) -> &Cell {
        &self.field[index]
    }
    
    fn get_cell_by_index_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.field[index]
    }

    pub fn calculate_target(&self, start_id: usize, direction: Direction) -> Option<usize> {
        let row = start_id / self.cols; // Поточний рядок
        let col = start_id % self.cols; // Поточний стовпець

        let (new_row, new_col) = match direction {
            Direction::Top => (
                if row > 0 { row - 1 } else { row },
                col,
            ),
            Direction::Bottom => (
                if row < self.rows - 1 { row + 1 } else { row },
                col,
            ),
            Direction::Left => (
                row,
                if col > 0 { col - 1 } else { col },
            ),
            Direction::Right => (
                row,
                if col < self.cols - 1 { col + 1 } else { col },
            ),
            Direction::TopRight => {
                let new_row = if row > 0 { row - 1 } else { row };
                let new_col = if col < self.cols - 1 { col + 1 } else { col };

                if new_row == row && new_col != col {
                    (row, new_col)
                } else if new_col == col && new_row != row {
                    (new_row, col)
                } else {
                    (new_row, new_col)
                }
            }
            Direction::TopLeft => {
                let new_row = if row > 0 { row - 1 } else { row };
                let new_col = if col > 0 { col - 1 } else { col };

                if new_row == row && new_col != col {
                    (row, new_col)
                } else if new_col == col && new_row != row {
                    (new_row, col)
                } else {
                    (new_row, new_col)
                }
            }
            Direction::BottomRight => {
                let new_row = if row < self.rows - 1 { row + 1 } else { row };
                let new_col = if col < self.cols - 1 { col + 1 } else { col };

                if new_row == row && new_col != col {
                    (row, new_col)
                } else if new_col == col && new_row != row {
                    (new_row, col)
                } else {
                    (new_row, new_col)
                }
            }
            Direction::BottomLeft => {
                let new_row = if row < self.rows - 1 { row + 1 } else { row };
                let new_col = if col > 0 { col - 1 } else { col };

                if new_row == row && new_col != col {
                    (row, new_col)
                } else if new_col == col && new_row != row {
                    (new_row, col)
                } else {
                    (new_row, new_col)
                }
            }
        };

        if new_row < self.rows && new_col < self.cols {
            Some(new_row * self.cols + new_col)
        } else {
            None
        }
    }
    
    
}
