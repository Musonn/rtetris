# RTetris

A simple Tetris game implemented in Rust using the Yew framework.

## Features

- **Tetromino Rotation**: Rotate tetrominoes with smooth transitions.
- **Random Tetromino Spawn**: Generate random tetromino shapes for gameplay.
- **Wall Kicks (Optional)**: Implement a basic "wall kick" system for smoother rotations near walls.

## TODO

### Wall Kicks (Optional)
For a smoother user experience, implement a basic "wall kick" system:
1. Try rotating the tetromino in place.
2. If blocked, try nudging the tetromino left or right by 1 block.
3. If still blocked, cancel the rotation.

### Random Tetromino Spawn
Ensure that new tetrominoes spawn randomly from the available shapes (`I`, `O`, `T`, `S`, `Z`, `J`, `L`).