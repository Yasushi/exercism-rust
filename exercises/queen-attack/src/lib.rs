
pub struct ChessPosition {
    x: isize,
    y: isize,
}

impl ChessPosition {
    pub fn new(x: isize, y: isize) -> Result<Self, ()> {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            Ok(Self { x: x, y: y })
        } else {
            Err(())
        }
    }
}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Self {
        Self { pos: pos }
    }

    pub fn can_attack(&self, t: &Self) -> bool {
        (self.pos.x - t.pos.x).abs() == (self.pos.y - t.pos.y).abs() || self.pos.x == t.pos.x ||
            self.pos.y == t.pos.y
    }
}
