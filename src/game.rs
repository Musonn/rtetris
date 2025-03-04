pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Filled(u8), // Represents different block colors
}

pub struct Board {
    grid: [[Cell; WIDTH]; HEIGHT], // Fixed-size 2D array
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == Cell::Empty
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: Cell) {
        self.grid[y][x] = value;
    }

    pub fn clear_full_lines(&mut self) {
        let mut new_grid = [[Cell::Empty; WIDTH]; HEIGHT]; // Temporary cleared grid
        let mut new_row = HEIGHT as isize - 1; // Start from the bottom

        // Iterate from bottom to top, shifting rows down if they are not full
        for y in (0..HEIGHT).rev() {
            if self.grid[y].iter().any(|&cell| cell == Cell::Empty) {
                new_grid[new_row as usize] = self.grid[y]; // Copy row
                new_row -= 1; // Move up
            }
        }

        self.grid = new_grid; // Update board with the cleared version
    }
}
