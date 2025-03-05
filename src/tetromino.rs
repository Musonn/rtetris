#[derive(Clone, Copy)]
pub enum TetrominoType {
    I, O, T, S, Z, J, L,
}

pub const TETROMINOES: [[[i32; 2]; 4]; 7] = [
    // I
    [[0, 1], [1, 1], [2, 1], [3, 1]],
    // O
    [[0, 0], [1, 0], [0, 1], [1, 1]],
    // T
    [[1, 0], [0, 1], [1, 1], [2, 1]],
    // S
    [[1, 0], [2, 0], [0, 1], [1, 1]],
    // Z
    [[0, 0], [1, 0], [1, 1], [2, 1]],
    // J
    [[0, 0], [0, 1], [1, 1], [2, 1]],
    // L
    [[2, 0], [0, 1], [1, 1], [2, 1]],
];

#[derive(Clone)]
pub struct Tetromino {
    pub shape: TetrominoType,
    pub position: [i32; 2], // (x, y) position on the board
    pub cells: [[i32; 2]; 4], // Block positions
}

impl Tetromino {
    pub fn new(shape: TetrominoType) -> Self {
        let index = shape as usize;
        Self {
            shape,
            position: [3, 0], // Start near the top center
            cells: TETROMINOES[index],
        }
    }

    pub fn move_down(&mut self) {
        self.position[1] += 1;
    }

    pub fn move_left(&mut self) {
        self.position[0] -= 1;
    }

    pub fn move_right(&mut self) {
        self.position[0] += 1;
    }
}
