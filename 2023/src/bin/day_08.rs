use std::collections::HashMap;

use aoc_2023::get_input;
use itertools::Itertools;
use num::Integer;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(cha: char) -> Self {
        match cha {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction given: \"{}\"", cha),
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    name: String,
    left: String,
    right: String,
}

impl Waypoint {
    fn get_destination(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn main() {
    let input = get_input(8);
    let mut input = input.lines();
    let mut map: HashMap<String, Waypoint> = HashMap::new();
    let directions: Vec<Direction> = input
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect();
    input.next().unwrap();

    for line in input {
        let (waypoint_name, points) = line.split(" = ").collect_tuple().unwrap();
        let (left, right) = points
            .trim_matches(|c| c == ')' || c == '(')
            .split(", ")
            .collect_tuple()
            .unwrap();
        map.insert(
            waypoint_name.to_string(),
            Waypoint {
                name: waypoint_name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    let locations = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| map.get(x).unwrap())
        .collect_vec();
    let mut counts = vec![];

    for location in locations {
        let mut current_location = location;
        let mut moved = 0usize;
        for (idx, direction) in directions.iter().cycle().enumerate() {
            moved = idx;
            if current_location.name.ends_with('Z') {
                break;
            }
            current_location = map
                .get(current_location.get_destination(direction))
                .unwrap();
        }
        counts.push(moved)
    }

    dbg!(counts.into_iter().reduce(|a, b| a.lcm(&b)).unwrap());
}
