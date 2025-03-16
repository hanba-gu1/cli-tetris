#[derive(Clone, Copy)]
pub enum MinoOperation {
    Move(Direction),
    Fall,
    RotateLeft,
    RotateRight,
    Hold,
    HardDrop,
    SoftDrop,
    Change,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
