#[derive(Clone, Copy)]
pub enum MinoOperation {
    Move(Direction),
    Fall,
    RotateLeft,
    RotateRight,
    Hold,
    HardDrop,
    StartSoftDrop,
    EndSoftDrop,
    Change,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
