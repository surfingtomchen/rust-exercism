#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn advance_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::West => (-1, 0),
            Direction::South => (0, -1),
            Direction::East => (1, 0)
        }
    }
}

pub struct Robot {
    pos: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = self.direction.turn_right();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = self.direction.turn_left();
        self
    }

    pub fn advance(mut self) -> Self {
        let (x, y) = self.direction.advance_delta();
        self.pos.0 += x;
        self.pos.1 += y;
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, c| {
            match c {
                'A' => r.advance(),
                'L' => r.turn_left(),
                'R' => r.turn_right(),
                _ => r,
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
