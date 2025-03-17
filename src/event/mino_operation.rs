#[derive(Clone, Copy)]
pub(crate) enum MinoOperation {
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
pub(crate) enum Direction {
    Left,
    Right,
}
