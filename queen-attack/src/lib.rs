#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let check = |v: i32| v >= 0 && v < 8;
        if !check(rank) || !check(file) {
            return None;
        }
        Some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // rank or file
        if self.pos.file == other.pos.file || self.pos.rank == other.pos.rank {
            return true;
        }

        // diagonal
        if (self.pos.file - other.pos.file).abs() == (self.pos.rank - other.pos.rank).abs() {
            return true;
        }

        false
    }
}
