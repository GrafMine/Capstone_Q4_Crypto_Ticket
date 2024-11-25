use super::direction::Direction;

#[derive(Debug, Clone)]
pub struct Cell {
    pub current_color: String,
    pub next_color: Option<String>,
    pub move_direction: Option<Direction>,
    pub x: usize,
    pub y: usize,
    pub index: usize,
}

impl Cell {
    pub fn new(current_color: String, x: usize, y: usize, size: usize) -> Self {
        Cell {
            current_color,
            next_color: None,
            move_direction: None,
            x,
            y,
            index: y * size + x,
        }
    }

    pub fn create_cell(x: usize, y: usize, size: usize) -> Cell {
        let colors = vec!["red", "green", "blue", "yellow"];
        let color = colors[(y * size + x) % colors.len()].to_string();
        Cell::new(color, x, y, size)
    }
}
