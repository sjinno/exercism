#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::East,
                ..self
            },
            Direction::East => Self {
                direction: Direction::South,
                ..self
            },
            Direction::South => Self {
                direction: Direction::West,
                ..self
            },
            Direction::West => Self {
                direction: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::West,
                ..self
            },
            Direction::West => Self {
                direction: Direction::South,
                ..self
            },
            Direction::South => Self {
                direction: Direction::East,
                ..self
            },
            Direction::East => Self {
                direction: Direction::North,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self {
                position: (self.position.0, self.position.1 + 1),
                ..self
            },
            Direction::East => Self {
                position: (self.position.0 + 1, self.position.1),
                ..self
            },
            Direction::South => Self {
                position: (self.position.0, self.position.1 - 1),
                ..self
            },
            Direction::West => Self {
                position: (self.position.0 - 1, self.position.1),
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut curr = self;

        for i in instructions.chars() {
            match i {
                'R' => curr = curr.turn_right(),
                'L' => curr = curr.turn_left(),
                'A' => curr = curr.advance(),
                _ => {}
            }
        }
        curr
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
