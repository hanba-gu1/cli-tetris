#[derive(Clone)]
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

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
}
