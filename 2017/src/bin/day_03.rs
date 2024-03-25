use core::fmt;
use std::collections::HashMap;

use aoc_2017::get_input;

fn main() {
    let input = get_input(3);
    let input = input.parse::<usize>().unwrap();

    dbg!(part_1(input));
    dbg!(part_2(input));
}

fn part_1(input: usize) -> usize {
    let point = SpiralIterator::new().nth(input - 1).unwrap();
    point.x.unsigned_abs() + point.y.unsigned_abs()
}

fn part_2(input: usize) -> usize {
    let mut map = HashMap::new();
    let mut spiral_iter = SpiralIterator::new();
    spiral_iter.next();
    map.insert(Point { x: 0, y: 0 }, 1_usize);
    loop {
        let point = spiral_iter.next().unwrap();
        let value = point.neighbours()
            .into_iter()
            .map(|x| *map.get(&x).unwrap_or(&0))
            .sum();
        map.insert(point, value);

        if value >= input {
            return value;
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

impl Point {
    fn move_dir(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point { x: self.x, y: self.y + 1 },
            Direction::Down => Point { x: self.x, y: self.y - 1 },
            Direction::Left => Point { x: self.x - 1, y: self.y },
            Direction::Right => Point { x: self.x + 1, y: self.y },
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        vec![
            self.move_dir(Direction::Up),
            self.move_dir(Direction::Up).move_dir(Direction::Right),
            self.move_dir(Direction::Right),
            self.move_dir(Direction::Right).move_dir(Direction::Down),
            self.move_dir(Direction::Down),
            self.move_dir(Direction::Down).move_dir(Direction::Left),
            self.move_dir(Direction::Left),
            self.move_dir(Direction::Left).move_dir(Direction::Up),
        ]
    }
}

#[derive(Debug)]
struct SpiralIterator {
    current_direction: Direction,
    current_position: Point,
    side_length_minus_1: usize,
    i: usize,
}

impl SpiralIterator {
    fn new() -> Self {
        SpiralIterator {
            current_direction: Direction::Right,
            side_length_minus_1: 0,
            current_position: Point { x: 0, y: 0 },
            i: 1,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current_position;

        self.current_position = self.current_position.move_dir(self.current_direction);
        self.i -= 1;

        if self.i == 0 {
            match self.current_direction {
                Direction::Down => {
                    self.current_direction = Direction::Right;
                    // need to push into the next cycle
                    self.i = self.side_length_minus_1 + 1;
                },
                Direction::Right => {
                    self.current_direction = Direction::Up;
                    self.side_length_minus_1 += 2;
                    // we are already 1 above the bottom right corner
                    self.i = self.side_length_minus_1 - 1;
                },
                _ => {
                    self.current_direction = self.current_direction.next();
                    self.i = self.side_length_minus_1;
                },
            }
        }

        Some(out)
    }
}