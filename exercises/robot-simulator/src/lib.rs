// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Robot {
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Self {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let next = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            pos: self.pos,
            dir: next,
        }
    }

    pub fn turn_left(self) -> Self {
        let next = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            pos: self.pos,
            dir: next,
        }
    }

    pub fn advance(self) -> Self {
        let (dx, dy) = match self.dir {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };
        Self {
            pos: (self.pos.0 + dx, self.pos.1 + dy),
            dir: self.dir,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| {
            match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            }
        })
    }

    pub fn position(&self) -> (isize, isize) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
