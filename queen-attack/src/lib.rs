#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(i32, i32);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (a, b) if !(0..=7).contains(&a) || !(0..=7).contains(&b) => None,
            _ => Some(ChessPosition { rank, file }),
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position.rank, position.file)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let is_same_rank = |other: &Queen| -> bool { self.0 == other.0 };
        let is_same_file = |other: &Queen| -> bool { self.1 == other.1 };
        let is_diagonal = |other: &Queen| -> bool {
            matches!((self.0 - other.0, self.1 - other.1), (a, b) if a.abs() == b.abs())
        };
        is_same_rank(other) || is_same_file(other) || is_diagonal(other)
    }
}
