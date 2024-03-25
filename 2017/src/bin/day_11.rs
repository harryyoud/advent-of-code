use aoc_2017::get_input;
use hex2d::{Coordinate, Direction};
use itertools::Itertools;

fn main() {
    let input = get_input(11);
    let directions = parse_input(&input);

    let (part_1, part_2) = solve(&directions);
    dbg!(part_1, part_2);
}

fn solve(directions: &[Direction]) -> (usize, usize) {
    let start = Coordinate::new(0isize, 0);
    let mut current = start.clone();
    let mut max_distance = 0usize;

    for dir in directions {
        current = current + *dir;
        max_distance = max_distance.max(start.distance(current).unsigned_abs());
    }

    (start.distance(current).unsigned_abs(), max_distance)
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.trim().split(",").map(|x| match x {
        "n" => Direction::YZ,
        "ne" => Direction::XZ,
        "se" => Direction::XY,
        "s" => Direction::ZY,
        "sw" => Direction::ZX,
        "nw" => Direction::YX,
        s => panic!("Invalid direction: {s}"),
    }).collect_vec()
}
