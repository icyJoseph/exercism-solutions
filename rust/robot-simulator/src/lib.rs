// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

// TODO: Find a way to DRY this code
impl Direction {
    fn turn_clockwise(self, clockwise: bool) -> Direction {
        match self {
            Direction::North => {
                if clockwise {
                    Direction::East
                } else {
                    Direction::West
                }
            }
            Direction::East => {
                if clockwise {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Direction::South => {
                if clockwise {
                    Direction::West
                } else {
                    Direction::East
                }
            }
            Direction::West => {
                if clockwise {
                    Direction::North
                } else {
                    Direction::South
                }
            }
        }
    }
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_point(self, direction: &Direction) -> Point {
        match direction {
            Direction::North => Point {
                y: self.y + 1,
                ..self
            },
            Direction::South => Point {
                y: self.y - 1,
                ..self
            },
            Direction::East => Point {
                x: self.x + 1,
                ..self
            },
            Direction::West => Point {
                x: self.x - 1,
                ..self
            },
        }
    }
}

pub struct Robot {
    start: Point,
    current: Point,
    move_direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            start: Point { x, y },
            current: Point { x, y },
            move_direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            start: self.start,
            current: self.current,
            move_direction: self.move_direction.turn_clockwise(true),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            move_direction: self.move_direction.turn_clockwise(false),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            current: self.current.move_point(&self.move_direction),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        // it is not absolutely needed to collect
        // because split already brings back an iterator
        // let instructions_list: Vec<&str> = instructions.split("").collect();
        let instructions_list = instructions.split("");
        let mut end_robot = Robot { ..self };

        for instruction in instructions_list {
            end_robot = match instruction {
                "R" => end_robot.turn_right(),
                "L" => end_robot.turn_left(),
                "A" => end_robot.advance(),
                _ => end_robot,
            }
        }
        end_robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.current.x, self.current.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.move_direction
    }
}
