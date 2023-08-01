use array2d::Array2D;
use rand::Rng;

#[derive(Debug)]
pub struct Board {
    cells: Array2D<u8>,
}

impl Board {
    pub fn new() -> Self {
        let random_cell = || -> u8 {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..=1)
        };

        let array = Array2D::filled_by_row_major(random_cell, 20, 20);
        Self { cells: array }
    }
}
