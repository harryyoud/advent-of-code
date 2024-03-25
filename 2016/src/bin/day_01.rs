use std::collections::HashSet;

use aoc_2016::get_input;

fn main() {
    let input = get_input(1);
    let movements = input.split(", ").map(|x| {
        let (direction, distance) = x.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        (direction.chars().next().unwrap(), distance)
    });

    let (part_1, part_2) = solve(movements);
    dbg!(part_1);
    dbg!(part_2);
}

fn solve(movements: impl Iterator<Item=(char, i32)>) -> (u32, u32) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut direction = Direction::North;
    let mut visited_twice = None;
    let mut x = 0i32;
    let mut y = 0i32;

    for movement in movements {
        direction = direction.turn(movement.0);

        for _ in 0..movement.1 {            
            match direction {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }
            if visited_twice.is_none() {
                if visited.contains(&(x, y)) {
                    visited_twice = Some(x.abs() as u32 + y.abs() as u32)
                }
                visited.insert((x, y));
            }
        }
    }

    (x.abs() as u32 + y.abs() as u32, visited_twice.unwrap())
}

enum Direction {
    North, South, East, West
}

impl Direction {
    fn turn(self, lr: char) -> Direction {
        match lr {
            'L' => match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            'R' => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            s => panic!("Invalid direction: {s}"),
        }
    }
}