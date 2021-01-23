#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[rustfmt::skip]
    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Self { direction: Direction::East,  ..self },
            Direction::East  => Self { direction: Direction::South, ..self },
            Direction::South => Self { direction: Direction::West,  ..self },
            Direction::West  => Self { direction: Direction::North, ..self },
        }
    }

    #[rustfmt::skip]
    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Self { direction: Direction::West,  ..self },
            Direction::West  => Self { direction: Direction::South, ..self },
            Direction::South => Self { direction: Direction::East,  ..self },
            Direction::East  => Self { direction: Direction::North, ..self },
        }
    }

    #[rustfmt::skip]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self { y: self.y + 1, ..self },
            Direction::East =>  Self { x: self.x + 1, ..self },
            Direction::South => Self { y: self.y - 1, ..self },
            Direction::West =>  Self { x: self.x - 1, ..self },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
