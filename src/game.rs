use crate::tetromino::{Tetromino, TetrominoType};
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use yew::{Html, html};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    grid: [[bool; WIDTH]; HEIGHT],       // true = filled
    tetromino: Option<Tetromino>,        // Active tetromino
    ghost_tetromino: Option<Tetromino>,  // Where the active tetromino will land  (ghost)
    next_queue: VecDeque<TetrominoType>, // Upcoming tetrominos
    score: usize,                        // Cleared lines score
    is_game_over: bool,
}

impl Board {
    // --- Initialization & Tetromino Queue ---
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[false; WIDTH]; HEIGHT],
            tetromino: None,
            ghost_tetromino: None,
            next_queue: VecDeque::with_capacity(7),
            score: 0,
            is_game_over: false,
        };
        board.refill_bag();
        board.spawn_tetromino();
        board.predict_tetromino();
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
        self.next_queue.extend(types.into_iter());
    }

    pub fn spawn_tetromino(&mut self) {
        let shape = self.next_queue.pop_front().unwrap();
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
        if self.is_game_over {
            return;
        }
        if let Some(active_clone) = self.tetromino.clone() {
            let mut candidate = active_clone.clone();
            candidate.move_down();
            if self.is_collision(&candidate) {
                self.lock_tetromino();
                self.clear_full_lines();
                self.spawn_tetromino();
                if let Some(t) = &self.tetromino {
                    if self.is_collision(t) {
                        self.set_game_over(true);
                    }
                }
            } else if let Some(active) = &mut self.tetromino {
                active.move_down();
            }
        }
        self.predict_tetromino();
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

    pub fn set_game_over(&mut self, game_over: bool) {
        self.is_game_over = game_over;
        if game_over {
            self.tetromino = None;
        }
    }

    pub fn get_game_over(&self) -> bool {
        self.is_game_over
    }

    // --- Cell & Collision Helpers ---
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

    // --- Tetromino Placement ---
    pub fn predict_tetromino(&mut self) {
        // Predict landing position
        if let Some(tetromino) = &mut self.tetromino {
            let mut landing_tetromino = tetromino.clone();
            while !self.is_collision(&landing_tetromino) {
                landing_tetromino.move_down();
            }
            // Move back up one position
            landing_tetromino.position[1] -= 1;
            self.ghost_tetromino = Some(landing_tetromino);
        }
    }

    fn lock_tetromino(&mut self) {
        if let Some(tetromino) = &self.tetromino {
            for cell in tetromino.cells.iter() {
                let x = (cell[0] + tetromino.position[0]) as usize;
                let y = (cell[1] + tetromino.position[1]) as usize;
                if x < WIDTH && y < HEIGHT {
                    self.grid[y][x] = true;
                }
            }
        }
    }

    // --- Score & Next Tetromino ---
    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_next_tetromino(&self) -> TetrominoType {
        *self.next_queue.front().unwrap()
    }

    pub fn get_next_tetromino_cells(&self) -> [[i32; 2]; 4] {
        crate::tetromino::TETROMINO_ROTATIONS[self.get_next_tetromino() as usize][0]
    }

    pub fn is_next_preview_cell(&self, x: usize, y: usize) -> bool {
        for cell in self.get_next_tetromino_cells() {
            if cell[0] == x as i32 && cell[1] == y as i32 {
                return true;
            }
        }
        false
    }

    // --- Board Rendering Helpers ---
    pub fn render_static(&self) -> Html {
        html! {
            <div class="board static-layer">
                { for (0..HEIGHT).map(|y| html! {
                    <div class="row">
                        { for (0..WIDTH).map(|x| {
                            let filled = self.grid[y][x];
                            let class = if filled { "cell filled" } else { "cell empty" };
                            html! { <div class={class}></div> }
                        })}
                    </div>
                })}
            </div>
        }
    }

    pub fn render_active(&self) -> Html {
        let mut active_cells = Vec::new();

        // Collect ghost cells
        if let Some(predicted) = &self.ghost_tetromino {
            for cell in predicted.cells.iter() {
                let x = predicted.position[0] + cell[0];
                let y = predicted.position[1] + cell[1];
                if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    active_cells.push((x as usize, y as usize, "ghost"));
                }
            }
        }

        // Collect active cells (will override ghost if overlap)
        if let Some(active) = &self.tetromino {
            for cell in active.cells.iter() {
                let x = active.position[0] + cell[0];
                let y = active.position[1] + cell[1];
                if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    // Remove ghost at this position if exists
                    active_cells.retain(|&(cx, cy, _)| !(cx == x as usize && cy == y as usize));
                    active_cells.push((x as usize, y as usize, "active"));
                }
            }
        }

        html! {
            <div class="active-layer">
                { for active_cells.iter().map(|(x, y, class_type)| {
                    let left = 10 + x * 21; // 10px padding + x * (20px cell + 1px gap)
                    let top = 10 + y * 21;  // 10px padding + y * (20px cell + 1px gap)
                    let style = format!("position: absolute; left: {}px; top: {}px; width: 20px; height: 20px; border-radius: 3px;", left, top);
                    let class = format!("cell {}", class_type);
                    html! { <div class={class} style={style}></div> }
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
                            let filled = self.is_next_preview_cell(x, y);
                            let class = if filled { "cell filled" } else { "cell empty" };
                            html! { <div class={class}></div> }
                        })}
                    </div>
                })}
            </div>
        }
    }
}
