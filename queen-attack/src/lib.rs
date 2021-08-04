#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank, file)),
            _ => None
        }
    }

    pub fn same_rank(&self, other: &ChessPosition) -> bool {
        self.0 == other.0
    }

    pub fn same_file(&self, other: &ChessPosition) -> bool {
        self.1 == other.1
    }

    pub fn share_diagonal(&self, other: &ChessPosition) -> bool {
        (self.0 - other.0).abs() == (self.1 - other.1).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.same_file(&other.position)
            || self.position.same_rank(&other.position)
            || self.position.share_diagonal(&other.position)
    }
}
