#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub const TETROMINO_ROTATIONS: [[[[i32; 2]; 4]; 4]; 7] = [
    // I shape
    [
        [[0, 1], [1, 1], [2, 1], [3, 1]], // 0°
        [[2, 0], [2, 1], [2, 2], [2, 3]], // 90°
        [[0, 2], [1, 2], [2, 2], [3, 2]], // 180°
        [[1, 0], [1, 1], [1, 2], [1, 3]], // 270°
    ],
    // O shape
    [
        [[0, 0], [1, 0], [0, 1], [1, 1]], // 0°
        [[0, 0], [1, 0], [0, 1], [1, 1]], // 90°
        [[0, 0], [1, 0], [0, 1], [1, 1]], // 180°
        [[0, 0], [1, 0], [0, 1], [1, 1]], // 270°
    ],
    // T shape
    [
        [[1, 0], [0, 1], [1, 1], [2, 1]], // 0°
        [[1, 0], [1, 1], [1, 2], [0, 1]], // 90°
        [[0, 1], [1, 1], [2, 1], [1, 2]], // 180°
        [[1, 0], [1, 1], [1, 2], [2, 1]], // 270°
    ],
    // S shape
    [
        [[1, 0], [2, 0], [0, 1], [1, 1]], // 0°
        [[0, 0], [0, 1], [1, 1], [1, 2]], // 90°
        [[1, 1], [2, 1], [0, 2], [1, 2]], // 180°
        [[1, 0], [1, 1], [2, 1], [2, 2]], // 270°
    ],
    // Z shape
    [
        [[0, 0], [1, 0], [1, 1], [2, 1]], // 0°
        [[1, 0], [0, 1], [1, 1], [0, 2]], // 90°
        [[0, 1], [1, 1], [1, 2], [2, 2]], // 180°
        [[2, 0], [1, 1], [2, 1], [1, 2]], // 270°
    ],
    // J shape
    [
        [[0, 0], [0, 1], [1, 1], [2, 1]], // 0°
        [[1, 0], [2, 0], [1, 1], [1, 2]], // 90°
        [[0, 1], [1, 1], [2, 1], [2, 2]], // 180°
        [[1, 0], [1, 1], [1, 2], [0, 2]], // 270°
    ],
    // L shape
    [
        [[2, 0], [0, 1], [1, 1], [2, 1]], // 0°
        [[1, 0], [1, 1], [1, 2], [2, 2]], // 90°
        [[0, 1], [1, 1], [2, 1], [0, 2]], // 180°
        [[0, 0], [1, 0], [1, 1], [1, 2]], // 270°
    ],
];

#[derive(Clone)]
pub struct Tetromino {
    pub shape: TetrominoType,
    pub position: [i32; 2],   // (x, y) position on the board
    pub rotation: usize,      // 0 to 3
    pub cells: [[i32; 2]; 4], // Block positions
}

impl Tetromino {
    pub fn new(shape: TetrominoType) -> Self {
        let index = shape as usize;
        Self {
            shape,
            position: [3, 0], // Start near the top center
            rotation: 0,
            cells: TETROMINO_ROTATIONS[index][0],
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

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
        self.cells = TETROMINO_ROTATIONS[self.shape as usize][self.rotation];
    }
}
