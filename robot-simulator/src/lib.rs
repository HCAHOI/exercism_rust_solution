// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            d: new_d,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            d: new_d,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = (self.x, self.y);
        let new_pos: (i32, i32) = match self.d {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
        Self {
            x: new_pos.0,
            y: new_pos.1,
            d: self.d,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut status = self;
        for c in instructions.chars() {
            status = match c {
                'R' => status.turn_right(),
                'L' => status.turn_left(),
                'A' => status.advance(),
                _ => panic!(),
            }
        }
        status
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
