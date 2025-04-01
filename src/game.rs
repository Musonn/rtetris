use yew::prelude::*;
use crate::tetromino::{Tetromino, TetrominoType};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    grid: [[bool; WIDTH]; HEIGHT], // Fixed-size 2D array; true = filled
    tetromino: Option<Tetromino>,// Active tetromino (None if no active piece)
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[false; WIDTH]; HEIGHT],
            tetromino: None,
        };
    
        board.spawn_tetromino(TetrominoType::I);
        board.place_tetromino();

        board
    }

    pub fn spawn_tetromino(&mut self, shape: TetrominoType) {
        self.tetromino = Some(Tetromino::new(shape));
    }

    pub fn update(&mut self) {
        // Clear the current tetromino from the grid
        self.clear_tetromino();

        // Move the active tetromino down if it exists
        if let Some(tetromino) = &mut self.tetromino {
            tetromino.move_down();
        }

        // Place the tetromino back on the grid in its new position
        self.place_tetromino();
    }

    pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == false
    }

    pub fn set_cell(&mut self, x: usize, y: usize, filled: bool) {
        if x < WIDTH && y < HEIGHT {
            self.grid[y][x] = filled;
        } else {
            panic!("Coordinates out of bounds");
        }
    }

    pub fn clear_full_lines(&mut self) {
        let mut new_grid = [[false; WIDTH]; HEIGHT]; // Start with empty grid
        let mut new_row = HEIGHT as isize - 1; // Start from the bottom

        // Iterate from bottom to top, shifting rows down if they are not full
        for y in (0..HEIGHT).rev() {
            if self.grid[y].iter().any(|&cell| !cell) {
                new_grid[new_row as usize] = self.grid[y]; // Copy row
                new_row -= 1; // Move up
            }
        }

        self.grid = new_grid; // Update board with the cleared version
    }

    pub fn clear_grid(&mut self) {
        self.grid = [[false; WIDTH]; HEIGHT]; // Reset the grid to empty
    }

    pub fn place_tetromino(&mut self) {
    // Set the cells of the tetromino on the grid
    if let Some(tetromino) = &self.tetromino {
        let cells = tetromino.cells.clone(); // Clone the cells to avoid borrowing `self`
        let position = tetromino.position;  // Copy the position

        for cell in cells.iter() {
            let x = (cell[0] + position[0]) as usize;
            let y = (cell[1] + position[1]) as usize;
            self.set_cell(x, y, true); // Use set_cell to mark the cell as filled
            }
        }
    }

    pub fn clear_tetromino(&mut self) {
        // Clear the tetromino from the grid
        if let Some(tetromino) = &self.tetromino {
            let cells = tetromino.cells.clone(); // Clone the cells to avoid borrowing `self`
            let position = tetromino.position;  // Copy the position

            for cell in cells.iter() {
                let x = (cell[0] + position[0]) as usize;
                let y = (cell[1] + position[1]) as usize;
                self.set_cell(x, y, false); // Use set_cell to mark the cell as empty
            }
        }
    }

pub fn render(&self) -> Html {
    html! {
        <div class="board">
            <p>{"If you see this, Yew is rendering the component."}</p>
            { for self.grid.iter().map(|row| html! {
                <div class="row">
                    { for row.iter().map(|&cell| html! {
                        <div class={ if cell { "cell filled" } else { "cell empty" } }></div>
                    }) }
                </div>
            }) }
        </div>
    }
}

}
