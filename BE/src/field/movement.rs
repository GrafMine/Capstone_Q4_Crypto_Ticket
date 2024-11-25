use super::direction::Direction;
use rand::Rng;

pub struct Movement {
    pub index_cell: usize,
    pub direction: Direction,
    pub diff: usize,
}

impl Movement {
    pub fn new(index_cell: usize, direction: Direction, diff: usize) -> Self {
        Movement {
            index_cell,
            direction,
            diff,
        }
    }

    pub fn generate_movement(size: usize) -> usize {
        rand::thread_rng().gen_range(0..size * size)
    }
    
    pub fn validate_direction(diff: usize, index: usize) -> Direction {
        // 0 - up, 1 - right, 2 - down, 3 - left
    }
}