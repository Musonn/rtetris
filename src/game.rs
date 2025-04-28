use yew::prelude::*;
use crate::tetromino::{Tetromino, TetrominoType};
use rand::seq::SliceRandom;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    grid: [[bool; WIDTH]; HEIGHT], // Fixed-size 2D array; true = filled
    tetromino: Option<Tetromino>,// Active tetromino (None if no active piece)
    next_queue: Vec<TetrominoType>, // Queue of upcoming tetrominos
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[false; WIDTH]; HEIGHT],
            tetromino: None,
            next_queue: Vec::<TetrominoType>::new(),
        };
    
        board.spawn_tetromino();
        board.place_tetromino();

        board
    }

    fn refill_bag(&mut self) {
        let mut types = [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ];
        let mut rng = rand::rng();
        types.shuffle(&mut rng);
        self.next_queue.extend_from_slice(&types);
    }

    pub fn spawn_tetromino(&mut self) {
        if self.next_queue.is_empty() {
            self.refill_bag();
        }
        let shape = self.next_queue.remove(0); // Take from front of queue
        self.tetromino = Some(Tetromino::new(shape));
    }

    pub fn rotate_tetromino(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut next_tetromino = tetromino.clone();
    
            // Step 1: Try rotating in place
            next_tetromino.rotate();
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
    
            // Step 2: Try nudging left
            next_tetromino.position[0] -= 1;
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
    
            // Step 3: Try nudging right
            next_tetromino.position[0] += 2;
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
    
            // Step 4: Cancel rotation (do nothing)
        }
    }

    pub fn update(&mut self) {
        // Clear the current tetromino from the grid
        self.clear_tetromino();
    
        let mut landed = false;
    
        // Move the active tetromino down if it exists
        if let Some(tetromino) = &self.tetromino {
            // Check if the tetromino can move down
            for cell in &tetromino.cells {
                let x = (cell[0] + tetromino.position[0]) as usize;
                let y = (cell[1] + tetromino.position[1] + 1) as usize; // Check one row below
    
                if y >= HEIGHT || !self.is_cell_empty(x, y) {
                    landed = true;
                    break;
                }
            }
        }
        
        if let Some(tetromino) = &mut self.tetromino {
            if !landed {
                tetromino.move_down(); // Move down if not landed
            }
        }
    
        // Place the tetromino back on the grid in its new position
        self.place_tetromino();
    
        // If the tetromino has landed, spawn a new one
        if landed {
            self.spawn_tetromino(); // Spawn a new tetromino (you can randomize this)
        }
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

    pub fn is_collision(&self, tetromino: &Tetromino) -> bool {
        for cell in &tetromino.cells {
            let x = (cell[0] + tetromino.position[0]) as isize;
            let y = (cell[1] + tetromino.position[1]) as isize;

            if x < 0 || x >= WIDTH as isize || y < 0 || y >= HEIGHT as isize {
                return true; // Out of bounds
            }

            if y >= 0 && self.grid[y as usize][x as usize] {
                return true; // Collision with filled cell
            }
        }
        false
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

    pub fn move_tetromino_left(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut next_tetromino = tetromino.clone();
            next_tetromino.move_left();
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
            }
        }
    }
    
    pub fn move_tetromino_right(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut next_tetromino = tetromino.clone();
            next_tetromino.move_right();
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
            }
        }
    }
    
    pub fn move_tetromino_down(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut next_tetromino = tetromino.clone();
            next_tetromino.move_down();
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
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
