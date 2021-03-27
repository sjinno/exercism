#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(i32, i32);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position.rank, position.file)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0 == other.0
            || self.1 == other.1
            || matches!((self.0 - other.0, self.1 - other.1), (a, b) if a.abs() == b.abs())
    }
}
