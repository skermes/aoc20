use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Rain Risk";

#[derive(Debug)]
struct Vec2 {
    x: isize,
    y: isize
}

// We only ever turn in 90 degree increments.
// Normalize left and right turns to always be right.
#[derive(Debug, Copy, Clone)]
enum Degrees {
    Zero = 0,
    Ninety = 1,
    OneEighty = 2,
    TwoSeventy = 3
}

impl Degrees {
    fn from_args(right: bool, mut n: isize) -> Result<Degrees, AocError> {
        if !right {
            n = 360 - n;
        }

        match n {
            0 => Ok(Degrees::Zero),
            90 => Ok(Degrees::Ninety),
            180 => Ok(Degrees::OneEighty),
            270 => Ok(Degrees::TwoSeventy),
            _ => Err(AocError::Misc("Invalid turn amount".to_string()))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Heading {
    North = 0,
    South = 2,
    East = 1,
    West = 3
}

const CLOCKWISE: [Heading; 4] =
    [Heading::North, Heading::East, Heading::South, Heading::West];
impl Heading {
    fn turn(&self, deg: &Degrees) -> Heading {
        CLOCKWISE[(*self as usize + *deg as usize) % CLOCKWISE.len()]
    }
}

#[derive(Debug)]
enum Instruction {
    Shift(Vec2),
    Turn(Degrees),
    // Forward moves can only ever be positive, but this saves us a bunch of
    // casting.
    Forward(isize)
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = &s[0..1];
        let amount: isize = s[1..].parse()?;

        match code {
            "N" => Ok(Instruction::Shift(Vec2{ x: 0, y: amount })),
            "S" => Ok(Instruction::Shift(Vec2{ x: 0, y: -amount })),
            "E" => Ok(Instruction::Shift(Vec2{ x: amount, y: 0 })),
            "W" => Ok(Instruction::Shift(Vec2{ x: -amount, y: 0 })),
            "L" => Ok(Instruction::Turn(Degrees::from_args(false, amount)?)),
            "R" => Ok(Instruction::Turn(Degrees::from_args(true, amount)?)),
            "F" => Ok(Instruction::Forward(amount)),
            _ => Err(AocError::Misc("Invalid instruction code".to_string()))
        }
    }
}

#[derive(Debug)]
struct Ferry {
    pos: Vec2,
    heading: Heading,
    waypoint: Vec2
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            pos: Vec2{ x: 0, y: 0 },
            heading: Heading::East,
            waypoint: Vec2{ x: 10, y: 1 }
        }
    }

    fn follow_instruction_p1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Shift(offset) => {
                self.pos.x += offset.x;
                self.pos.y += offset.y;
            },
            Instruction::Turn(deg) => {
                self.heading = self.heading.turn(deg);
            },
            Instruction::Forward(distance) => {
                match self.heading {
                    Heading::North => self.pos.y += distance,
                    Heading::South => self.pos.y -= distance,
                    Heading::East => self.pos.x += distance,
                    Heading::West => self.pos.x -= distance
                }
            }
        }
    }

    fn follow_instruction_p2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Shift(offset) => {
                self.waypoint.x += offset.x;
                self.waypoint.y += offset.y;
            },
            Instruction::Turn(deg) => {
                match deg {
                    Degrees::Zero => {},
                    Degrees::Ninety => {
                        let old_x = self.waypoint.x;
                        self.waypoint.x = self.waypoint.y;
                        self.waypoint.y = -old_x;
                    },
                    Degrees::OneEighty => {
                        self.waypoint.x = -self.waypoint.x;
                        self.waypoint.y = -self.waypoint.y;
                    },
                    Degrees::TwoSeventy => {
                        let old_x = self.waypoint.x;
                        self.waypoint.x = -self.waypoint.y;
                        self.waypoint.y = old_x;
                    }
                }
            },
            Instruction::Forward(times) => {
                self.pos.x += self.waypoint.x * times;
                self.pos.y += self.waypoint.y * times;
            }
        }
    }

    fn manhattan(&self) -> isize {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, AocError>>()?;

    let mut ferry = Ferry::new();
    for instruction in instructions {
        ferry.follow_instruction_p1(&instruction);
    }

    Ok(ferry.manhattan().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, AocError>>()?;

    let mut ferry = Ferry::new();
    for instruction in instructions {
        ferry.follow_instruction_p2(&instruction);
    }

    Ok(ferry.manhattan().to_string())
}