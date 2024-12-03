#![feature(iterator_try_collect)]

use aoc_2020::{get_input, Vec2d};

fn main() {
    let input = get_input(12);

    let instructions: Vec<Instruction> = input.lines()
        .map(Instruction::try_from)
        .try_collect()
        .unwrap();

    let ship = Ship {
        direction: Direction::East,
        position: (0isize, 0isize).into(),
        waypoint_offset: (10isize, 1isize).into(),
    };

    dbg!(part_1(ship.clone(), &instructions));
    dbg!(part_2(ship.clone(), &instructions));
}

fn part_1(mut ship: Ship, instructions: &[Instruction]) -> usize {
    for instruction in instructions {
        ship.apply_instruction_v1(*instruction);
    }

    ship.position.manhattan_origin()
}

fn part_2(mut ship: crate::Ship, instructions: &[crate::Instruction]) -> usize {
    for instruction in instructions {
        ship.apply_instruction_v2(*instruction);
    }

    ship.position.manhattan_origin()
}

#[derive(Clone, Debug)]
struct Ship {
    direction: Direction,
    position: Vec2d,
    waypoint_offset: Vec2d,
}

impl Ship {
    fn apply_instruction_v1(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft(degrees) => {
                self.direction = self.direction.counterclockwise_deg(degrees);
            },
            Instruction::TurnRight(degrees) => {
                self.direction = self.direction.clockwise_deg(degrees);
            },
            Instruction::MoveDirection(direction, amount) => {
                self.position = move_n(self.position, direction, amount)
            }
            Instruction::MoveForwards(amount) => {
                self.position = move_n(self.position, self.direction, amount)
            }
        }
    }

    fn apply_instruction_v2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft(degrees) => {
                for _ in 0..((degrees / 90) % 4) {
                    // rotating a point counterclockwise round the origin results in (x, y) -> (-y, x)
                    self.waypoint_offset = Vec2d {
                        x: -self.waypoint_offset.y,
                        y:  self.waypoint_offset.x
                    }
                }
            },
            Instruction::TurnRight(degrees) => {
                for _ in 0..((degrees / 90) % 4) {
                    // rotating a point clockwise round the origin results in (x, y) -> (y, -x)
                    self.waypoint_offset = Vec2d {
                        x:  self.waypoint_offset.y,
                        y: -self.waypoint_offset.x
                    }
                }
            },
            Instruction::MoveDirection(direction, amount) => {
                self.waypoint_offset = move_n(self.waypoint_offset, direction, amount)
            }
            Instruction::MoveForwards(amount) => {
                for _ in 0..amount {
                    self.position += self.waypoint_offset;
                }
            }
        }
    }
}

fn move_n(position: Vec2d, direction: Direction, amount: usize) -> Vec2d {
    use Direction::*;
    match direction {
        North => position.down_n(amount),
        East => position.right_n(amount),
        South => position.up_n(amount),
        West => position.left_n(amount),
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn clockwise(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn counterclockwise(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn clockwise_deg(&self, degrees: usize) -> Self {
        let turns = (degrees / 90) % 4;
        let mut out = self.clone();
        for _ in 0..turns {
            out = out.clockwise();
        }
        out
    }

    fn counterclockwise_deg(&self, degrees: usize) -> Self {
        let turns = (degrees / 90) % 4;
        let mut out = self.clone();
        for _ in 0..turns {
            out = out.counterclockwise();
        }
        out
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    TurnLeft(usize),
    TurnRight(usize),
    MoveDirection(Direction, usize),
    MoveForwards(usize),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Instruction::*;
        use Direction::*;
        let number = value[1..].parse::<usize>().unwrap();
        Ok(match value.chars().next() {
            None => return Err(()),
            Some('N') => MoveDirection(North, number),
            Some('E') => MoveDirection(East, number),
            Some('S') => MoveDirection(South, number),
            Some('W') => MoveDirection(West, number),
            Some('L') => TurnLeft(number),
            Some('R') => TurnRight(number),
            Some('F') => MoveForwards(number),
            Some(_) => return Err(()),
        })
    }
}
