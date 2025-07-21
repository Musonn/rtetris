use crate::tetromino::{Tetromino, TetrominoType};
use rand::seq::SliceRandom;
use yew::{Html, html};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    grid: [[bool; WIDTH]; HEIGHT], // true = filled
    tetromino: Option<Tetromino>,  // Active tetromino
    next_queue: Vec<TetrominoType>, // Upcoming tetrominos
    score: usize, // Cleared lines score
}

impl Board {
    // --- Initialization & Tetromino Queue ---
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[false; WIDTH]; HEIGHT],
            tetromino: None,
            next_queue: Vec::new(),
            score: 0,
        };
        board.refill_bag();
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
        let mut rng = rand::thread_rng();
        types.shuffle(&mut rng);
        self.next_queue.extend_from_slice(&types);
    }

    pub fn spawn_tetromino(&mut self) {
        let shape = self.next_queue.remove(0);
        self.tetromino = Some(Tetromino::new(shape));
        if self.next_queue.is_empty() {
            self.refill_bag();
        }
    }

    // --- Tetromino Movement & Rotation ---
    pub fn rotate_tetromino(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut next_tetromino = tetromino.clone();
            next_tetromino.rotate();
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
            next_tetromino.position[0] -= 1;
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
            next_tetromino.position[0] += 2;
            if !self.is_collision(&next_tetromino) {
                self.tetromino = Some(next_tetromino);
                return;
            }
            // Cancel rotation if all fail
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

    // --- Board State & Updates ---
    pub fn update(&mut self) {
        self.clear_tetromino();
        let mut landed = false;
        if let Some(tetromino) = &self.tetromino {
            for cell in &tetromino.cells {
                let x = (cell[0] + tetromino.position[0]) as usize;
                let y = (cell[1] + tetromino.position[1] + 1) as usize;
                if y >= HEIGHT || !self.is_cell_empty(x, y) {
                    landed = true;
                    break;
                }
            }
        }
        if let Some(tetromino) = &mut self.tetromino {
            if !landed {
                tetromino.move_down();
            }
        }
        self.place_tetromino();
        if landed {
            self.spawn_tetromino();
        }
    }

    pub fn clear_full_lines(&mut self) {
        let mut new_grid = [[false; WIDTH]; HEIGHT];
        let mut new_row = HEIGHT as isize - 1;
        let mut lines_cleared = 0;
        for y in (0..HEIGHT).rev() {
            if self.grid[y].iter().any(|&cell| !cell) {
                new_grid[new_row as usize] = self.grid[y];
                new_row -= 1;
            } else {
                lines_cleared += 1;
            }
        }
        self.grid = new_grid;
        self.score += lines_cleared * 100;
    }

    pub fn clear_grid(&mut self) {
        self.grid = [[false; WIDTH]; HEIGHT];
    }

    // --- Cell & Collision Helpers ---
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
                return true;
            }
            if y >= 0 && self.grid[y as usize][x as usize] {
                return true;
            }
        }
        false
    }

    // --- Tetromino Placement & Clearing ---
    pub fn place_tetromino(&mut self) {
        if let Some(tetromino) = &self.tetromino {
            let cells = tetromino.cells.clone();
            let position = tetromino.position;
            for cell in cells.iter() {
                let x = (cell[0] + position[0]) as usize;
                let y = (cell[1] + position[1]) as usize;
                self.set_cell(x, y, true);
            }
        }
    }

    pub fn clear_tetromino(&mut self) {
        if let Some(tetromino) = &self.tetromino {
            let cells = tetromino.cells.clone();
            let position = tetromino.position;
            for cell in cells.iter() {
                let x = (cell[0] + position[0]) as usize;
                let y = (cell[1] + position[1]) as usize;
                self.set_cell(x, y, false);
            }
        }
    }

    // --- Score & Next Tetromino ---
    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_next_tetromino(&self) -> Option<TetrominoType> {
        self.next_queue.first().copied()
    }

    pub fn get_next_tetromino_cells(&self) -> Option<[[i32; 2]; 4]> {
        self.get_next_tetromino().map(|next| {
            let index = next as usize;
            crate::tetromino::TETROMINO_ROTATIONS[index][0]
        })
    }

    // --- Board Rendering Helpers ---
    pub fn get_board_with_tetromino(&self) -> [[bool; WIDTH]; HEIGHT] {
        let mut board = self.grid;
        if let Some(tetromino) = &self.tetromino {
            for cell in tetromino.cells.iter() {
                let x = tetromino.position[0] + cell[0];
                let y = tetromino.position[1] + cell[1];
                if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    board[y as usize][x as usize] = true;
                }
            }
        }
        board
    }

    pub fn render(&self) -> Html {
        let board = self.get_board_with_tetromino();
        html! {
            <div class="board">
                { for (0..HEIGHT).map(|y| html! {
                    <div class="row">
                        { for (0..WIDTH).map(|x| {
                            let filled = board[y][x];
                            let class = if filled { "cell filled" } else { "cell empty" };
                            html! { <div class={class}></div> }
                        })}
                    </div>
                })}
            </div>
        }
    }

    pub fn render_next_tetromino(&self) -> Html {
        html! {
            <div class="next-tetromino">
                { for (0..4).map(|y| html! {
                    <div class="row">
                        { for (0..4).map(|x| {
                            let filled = self.next_tetromino_cell(x, y);
                            let class = if filled { "cell filled" } else { "cell empty" };
                            html! { <div class={class}></div> }
                        })}
                    </div>
                })}
            </div>
        }
    }

    pub fn next_tetromino_cell(&self, x: usize, y: usize) -> bool {
        if let Some(cells) = self.get_next_tetromino_cells() {
            for cell in &cells {
                if cell[0] == x as i32 && cell[1] == y as i32 {
                    return true;
                }
            }
        }
        false
    }
}
