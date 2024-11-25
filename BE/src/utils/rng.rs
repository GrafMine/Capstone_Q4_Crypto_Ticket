use rand::Rng;

pub fn generate_random_index(size: usize) -> usize {
    rand::thread_rng().gen_range(0..size * size)
}

pub fn generate_random_direction() -> usize {
    rand::thread_rng().gen_range(0..4)
}
