# RTetris

A simple Tetris game implemented in Rust using the Yew framework.

## Features

- **Tetromino Rotation**: Rotate tetrominoes with smooth transitions.
- **Random Tetromino Spawn**: Generate random tetromino shapes for gameplay.
- **Wall Kicks (Optional)**: Implement a basic "wall kick" system for smoother rotations near walls.

## TODO

### Advanced Wall Kick
When a rotation is attempted, 5 positions are sequentially tested (inclusive of basic rotation); if none are available, the rotation fails completely.

### Random Tetromino Spawn
Ensure that new tetrominoes spawn randomly from the available shapes (`I`, `O`, `T`, `S`, `Z`, `J`, `L`).