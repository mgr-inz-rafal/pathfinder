#[derive(Debug)]
pub enum PathfindingError {
    OutOfMap
}

#[derive(Debug)]
pub enum MapError {
    SizeMismatch,
    StartOutOfBounds,
    DestinationOutOfBounds,
    StartEqEnd,
    TooBig
}
