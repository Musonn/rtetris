# RTetris

A simple Tetris game implemented in Rust using the Yew framework.

## Features

- **Tetromino Rotation**: Rotate tetrominoes with smooth transitions.
- **Random Tetromino Spawn**: Generate random tetromino shapes for gameplay.
- **Next Tetris Display**: Show the next tetromino coming down

## TODO

### Advanced Wall Kick
When a rotation is attempted, 5 positions are sequentially tested (inclusive of basic rotation); if none are available, the rotation fails completely.

### Show the Next Tetromino
Add a peek_next() to show the upcoming piece.

### Scoring System by Level
Implement the original Nintendo scoring system, where points awarded for clearing lines increase with the game level and the number of lines cleared at once.