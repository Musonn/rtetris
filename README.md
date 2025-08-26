
# RTetris

https://github.com/user-attachments/assets/980b9123-7105-48a1-94a3-7a08d767a6a9

A simple Tetris game implemented in Rust using the [Yew](https://yew.rs/) framework for WebAssembly.

## Features

- **Classic Tetris Gameplay**: Move, rotate, and drop tetrominoes on a 10x20 board.
- **Tetromino Rotation**: Rotate pieces with wall kick logic.
- **Next Tetromino Preview**: See which piece is coming up next.
- **Score Tracking**: Score increases as you clear lines.
- **Keyboard Controls**: Use arrow keys to play (← → ↓ to move, ↑ to rotate).

## Controls

- **Left/Right Arrow**: Move tetromino left/right
- **Down Arrow**: Move tetromino down
- **Up Arrow**: Rotate tetromino

## How to Build & Run

1. **Install Rust** (if you don't have it): https://rustup.rs/
2. **Install Trunk** (for building Yew apps):
   ```sh
   cargo install trunk
   ```
3. **Build and run locally:**
   ```sh
   trunk serve --open
   ```
   This will open the game in your browser at `http://localhost:8080`.

## TODO

- Advanced wall kick logic for rotations
- Scoring system by level (classic NES rules)
- fix bug: keeping pressing ← → ↑ will keep the tetrimino afloat
